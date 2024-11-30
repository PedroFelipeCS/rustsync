mod logger;
mod structdir;

use chrono::Local;
use clap::Parser;
use dotenv::dotenv;
use log::{error, info, warn};
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use tar::Builder as TarBuilder;
use walkdir::WalkDir;
use structdir::backup_struct::create_backup_structure;

/// Estrutura que define os argumentos da linha de comando
#[derive(Parser)]
#[command(name = "rustsync")]
#[command(about = "Um sistema de backup de diretório", long_about = None)]
struct Cli {
    /// Diretório de origem
    #[arg(short = 's', long)]
    source: String,

    /// Diretório de destino
    #[arg(short = 'd', long)]
    destination: String,

    /// Nome do arquivo de backup
    #[arg(short = 'n', long, default_value = "backup")]
    backup_name: String,

    /// Modo verbose
    #[arg(short = 'v', long)]
    verbose: bool,
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
fn backup_directory(source: &str, destination: &str, backup_name: &str, verbose: bool) -> io::Result<()> {
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

    info!("Iniciando o backup de '{}' para '{}'", source, tar_path.display());

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
    Ok(())
}

/// Função principal que inicializa o logger, parseia os argumentos da linha de comando
/// e chama a função de backup.
///
/// # Erros
///
/// Registra um erro se ocorrer algum problema durante o processo de backup.
fn main() {
    dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env

    // Inicializar o logger
    logger::log::init_logger();

    let cli = Cli::parse();

    if let Err(e) = backup_directory(&cli.source, &cli.destination, &cli.backup_name, cli.verbose) {
        error!("Erro ao fazer backup: {}", e);
    }
}
