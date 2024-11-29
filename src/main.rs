use clap::Parser;
use chrono::Local;
use dotenv::dotenv;
use env_logger::Builder;
use env_logger::Env;
use log::{info, warn, error};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::SystemTime;
use tar::Builder as TarBuilder;
use walkdir::WalkDir;

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

fn backup_directory(source: &str, destination: &str, backup_name: &str, verbose: bool) -> io::Result<()> {
    // Obter a data atual e formatá-la
    let date = Local::now().format("%Y-%m-%d").to_string();
    
    // Adicionar a data e a extensão ao nome do arquivo de backup
    let backup_name_with_date = format!("{}_{}.tar", backup_name, date);
    
    let tar_path = PathBuf::from(destination).join(backup_name_with_date);
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

fn main() {
    dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env

    // Configurar o env_logger com um formato de timestamp personalizado
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let datetime = SystemTime::now();
            let datetime: chrono::DateTime<chrono::Local> = datetime.into();
            writeln!(
                buf,
                "{} [{}] - {}",
                datetime.format("%Y-%m-%d_%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    let cli = Cli::parse();

    if let Err(e) = backup_directory(&cli.source, &cli.destination, &cli.backup_name, cli.verbose) {
        error!("Erro ao fazer backup: {}", e);
    }
}
