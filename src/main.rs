mod config;
mod cron;
mod logger;
mod structdir;

use chrono::Local;
use clap::Parser;
use config::{load_config, BackupConfig, RcloneConfig};
use dotenv::dotenv;
use log::{error, info, warn};
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use structdir::backup_struct::create_backup_structure;
use tar::Builder as TarBuilder;
use walkdir::WalkDir;

/// Estrutura que define os argumentos da linha de comando
#[derive(Parser)]
#[command(name = "rustsync")]
#[command(about = "Um sistema de backup de diretório", long_about = None)]
struct Cli {
    /// Caminho para o arquivo de configuração
    #[arg(short = 'c', long)]
    config: Option<String>,

    /// Diretório de origem
    #[arg(short = 's', long)]
    source: Option<String>,

    /// Diretório de destino
    #[arg(short = 'd', long)]
    destination: Option<String>,

    /// Nome do arquivo de backup
    #[arg(short = 'n', long, default_value = "backup")]
    backup_name: String,

    /// Expressão cron para agendamento
    #[arg(short = 'r', long)]
    cron: Option<String>,

    /// Modo verbose
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Destino do rclone para sincronização
    #[arg(long)]
    rclone_dest: Option<String>,

    /// Expressão cron para sincronização com rclone
    #[arg(long)]
    rclone_cron: Option<String>,
}

/// Realiza o backup do diretório de origem para o destino.
///
/// # Argumentos
///
/// * `source` - Diretório de origem.
/// * `destination` - Diretório de destino.
/// * `backup_name` - Nome do arquivo de backup.
/// * `verbose` - Modo verbose para exibir mensagens detalhadas.
///
/// # Erros
///
/// Retorna um erro se ocorrer algum problema durante o processo de backup.
fn backup_directory(
    source: &str,
    destination: &str,
    backup_name: &str,
    verbose: bool,
) -> io::Result<PathBuf> {
    // Criar a estrutura do diretório de backup
    create_backup_structure(destination, backup_name)?;

    // Obter a hora atual e formatá-la
    let time = Local::now().format("%H-%M").to_string();

    // Adicionar a hora e a extensão ao nome do arquivo de backup
    let backup_name_with_time = format!("{}_{}.tar", backup_name, time);

    let tar_path = PathBuf::from(destination)
        .join(backup_name)
        .join(Local::now().format("%Y/%m/%d").to_string())
        .join(backup_name_with_time);
    let tar_file = File::create(&tar_path)?;
    let mut tar = TarBuilder::new(tar_file);

    info!(
        "Iniciando o backup de '{}' para '{}'",
        source,
        tar_path.display()
    );

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        match path.strip_prefix(source) {
            Ok(relative_path) => {
                if !relative_path.as_os_str().is_empty() {
                    tar.append_path_with_name(path, relative_path)?;
                    if verbose {
                        info!("Adicionando: {:?}", path);
                    }
                }
            }
            Err(e) => {
                warn!("Erro ao processar o caminho: {}", e);
            }
        }
    }

    tar.finish()?;
    info!("Backup concluído em: {}", tar_path.display());

    Ok(tar_path)
}

/// Sincroniza os arquivos de backup com a nuvem usando rclone.
///
/// # Argumentos
///
/// * `backup_path` - Caminho do arquivo de backup.
/// * `rclone_dest` - Destino do rclone para sincronização.
///
/// # Erros
///
/// Retorna um erro se ocorrer algum problema durante a sincronização.
fn sync_with_rclone(backup_path: &PathBuf, rclone_dest: &str) -> io::Result<()> {
    info!(
        "Sincronizando com a nuvem usando rclone para '{}'",
        rclone_dest
    );
    let output = Command::new("rclone")
        .arg("copy")
        .arg(backup_path.to_str().unwrap())
        .arg(rclone_dest)
        .output()?;

    if !output.status.success() {
        error!(
            "Erro ao sincronizar com a nuvem: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        info!("Sincronização com a nuvem concluída com sucesso.");
    }

    Ok(())
}

/// Função principal que inicializa o logger, parseia os argumentos da linha de comando
/// e chama a função de backup ou sincronização.
///
/// # Erros
///
/// Registra um erro se ocorrer algum problema durante o processo de backup ou sincronização.
fn main() {
    dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env

    // Inicializar o logger
    logger::log::init_logger();

    let cli = Cli::parse();

    if let Some(config_path) = &cli.config {
        let config = load_config(config_path);
        let rclone_config = Arc::new(config.rclone);
        for backup in config.backups {
            let backup = Arc::new(backup);
            cron::schedule_backups(backup.clone(), rclone_config.clone());
        }
        cron::schedule_rclone_sync(rclone_config.clone());
    } else {
        if let Some(cron_expr) = &cli.cron {
            let source = Arc::new(
                cli.source
                    .clone()
                    .expect("Diretório de origem não especificado"),
            );
            let destination = Arc::new(
                cli.destination
                    .clone()
                    .expect("Diretório de destino não especificado"),
            );
            let backup_name = Arc::new(cli.backup_name.clone());
            let cron_expr = Arc::new(cron_expr.clone());
            let rclone_dest = cli
                .rclone_dest
                .clone()
                .expect("Destino do rclone não especificado");
            let rclone_cron = cli
                .rclone_cron
                .clone()
                .expect("Expressão cron do rclone não especificada");
            let rclone_config = Arc::new(RcloneConfig {
                dest: rclone_dest,
                cron: rclone_cron,
            });
            cron::schedule_backups(
                Arc::new(BackupConfig {
                    source: source.to_string(),
                    destination: destination.to_string(),
                    backup_name: backup_name.to_string(),
                    cron: cron_expr.to_string(),
                    verbose: cli.verbose,
                }),
                rclone_config.clone(),
            );
            cron::schedule_rclone_sync(rclone_config.clone());
        } else {
            let source = cli
                .source
                .as_deref()
                .expect("Diretório de origem não especificado");
            let destination = cli
                .destination
                .as_deref()
                .expect("Diretório de destino não especificado");
            if let Ok(backup_path) =
                backup_directory(source, destination, &cli.backup_name, cli.verbose)
            {
                if let Some(rclone_dest) = &cli.rclone_dest {
                    if let Err(e) = sync_with_rclone(&backup_path, rclone_dest) {
                        error!("Erro ao sincronizar com a nuvem: {}", e);
                    }
                }
            } else {
                error!("Erro ao fazer backup");
            }
        }
    }

    // Manter o programa rodando para permitir que os threads de backup sejam executados
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
