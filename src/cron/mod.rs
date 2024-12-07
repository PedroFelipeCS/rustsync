use cron::Schedule;
use chrono::Local;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use crate::{backup_directory, sync_with_rclone};
use crate::config::{BackupConfig, RcloneConfig};

/// Agenda backups com base em uma expressão cron.
///
/// # Argumentos
///
/// * `config` - Configuração do backup.
/// * `rclone_config` - Configuraç��o global do rclone.
pub fn schedule_backups(config: Arc<BackupConfig>, rclone_config: Arc<RcloneConfig>) {
    let schedule = Schedule::from_str(&config.cron).expect("Erro ao parsear expressão cron");
    thread::spawn(move || {
        for datetime in schedule.upcoming(Local) {
            let now = Local::now();
            let duration = datetime.signed_duration_since(now).to_std().unwrap();
            thread::sleep(duration);
            if let Ok(backup_path) = backup_directory(&config.source, &config.destination, &config.backup_name, config.verbose) {
                if let Err(e) = sync_with_rclone(&backup_path, &rclone_config.dest) {
                    eprintln!("Erro ao sincronizar com a nuvem: {}", e);
                }
            } else {
                eprintln!("Erro ao fazer backup");
            }
        }
    });
}

/// Agenda a sincronização com o rclone com base em uma expressão cron.
///
/// # Argumentos
///
/// * `rclone_config` - Configuração global do rclone.
pub fn schedule_rclone_sync(rclone_config: Arc<RcloneConfig>) {
    let schedule = Schedule::from_str(&rclone_config.cron).expect("Erro ao parsear expressão cron");
    thread::spawn(move || {
        for datetime in schedule.upcoming(Local) {
            let now = Local::now();
            let duration = datetime.signed_duration_since(now).to_std().unwrap();
            thread::sleep(duration);
            if let Err(e) = sync_with_rclone(&PathBuf::from("."), &rclone_config.dest) {
                eprintln!("Erro ao sincronizar com a nuvem: {}", e);
            }
        }
    });
}