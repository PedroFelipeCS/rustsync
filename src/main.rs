use clap::{Arg, Parser};
use dotenv::dotenv;
use std::fs::File;
use std::io::{self};
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

    /// Modo verbose
    #[arg(short = 'v', long)]
    verbose: bool,
}

fn backup_directory(source: &str, destination: &str, verbose: bool) -> io::Result<()> {
    let tar_path = format!("{}/backup.tar", destination);
    let tar_file = File::create(&tar_path)?;
    let mut tar = Builder::new(tar_file);

    if verbose {
        println!("Iniciando o backup de '{}' para '{}'", source, destination);
    }

    // Caminhar pelo diretório de origem
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(source).unwrap();

        // Adicionar o arquivo ao arquivo tar somente se o caminho não estiver vazio
        if !relative_path.as_os_str().is_empty() {
            tar.append_path_with_name(path, relative_path)?;
            if verbose {
                println!("Adicionando: {:?}", path);
            }
        }
    }

    tar.finish()?;
    if verbose {
        println!("Backup concluído em: {}", tar_path);
    }
    Ok(())
}

fn main() {
    dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env

    // Parsear argumentos da linha de comando
    let cli = Cli::parse();

    // Chamar a função de backup
    if let Err(e) = backup_directory(&cli.source, &cli.destination, cli.verbose) {
        eprintln!("Erro ao fazer backup: {}", e);
    }
}
