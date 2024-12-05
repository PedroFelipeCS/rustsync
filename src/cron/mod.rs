use cron::Schedule;
use chrono::Local;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use crate::backup_directory;
use crate::config::BackupConfig;

/// Agenda backups com base em uma expressão cron.
///
/// # Argumentos
///
/// * `config` - Configuração do backup.
pub fn schedule_backups(config: Arc<BackupConfig>) {
    let schedule = Schedule::from_str(&config.cron).expect("Erro ao parsear expressão cron");
    thread::spawn(move || {
        for datetime in schedule.upcoming(Local) {
            let now = Local::now();
            let duration = datetime.signed_duration_since(now).to_std().unwrap();
            thread::sleep(duration);
            if let Err(e) = backup_directory(&config.source, &config.destination, &config.backup_name, config.verbose) {
                eprintln!("Erro ao fazer backup: {}", e);
            }
        }
    });
}