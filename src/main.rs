use clap::Parser;
use dotenv::dotenv;
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use tar::Builder;
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
    #[arg(short = 'n', long, default_value = "backup.tar")]
    backup_name: String,

    /// Modo verbose
    #[arg(short = 'v', long)]
    verbose: bool,
}

fn backup_directory(source: &str, destination: &str, backup_name: &str, verbose: bool) -> io::Result<()> {
    let tar_path = PathBuf::from(destination).join(backup_name);
    let tar_file = File::create(&tar_path)?;
    let mut tar = Builder::new(tar_file);

    if verbose {
        println!("Iniciando o backup de '{}' para '{}'", source, tar_path.display());
    }

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        match path.strip_prefix(source) {
            Ok(relative_path) => {
                if !relative_path.as_os_str().is_empty() {
                    tar.append_path_with_name(path, relative_path)?;
                    if verbose {
                        println!("Adicionando: {:?}", path);
                    }
                }
            }
            Err(e) => {
                eprintln!("Erro ao processar o caminho: {}", e);
            }
        }
    }

    tar.finish()?;
    if verbose {
        println!("Backup concluído em: {}", tar_path.display());
    }
    Ok(())
}

fn main() {
    dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env

    let cli = Cli::parse();

    if let Err(e) = backup_directory(&cli.source, &cli.destination, &cli.backup_name, cli.verbose) {
        eprintln!("Erro ao fazer backup: {}", e);
    }
}
