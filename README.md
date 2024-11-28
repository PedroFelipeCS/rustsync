# Rustsync

Rustsync é um sistema de backup de diretório escrito em Rust. Ele cria um arquivo tar contendo todos os arquivos e diretórios do diretório de origem especificado.

## Instalação

Para compilar e instalar o `rustsync`, você precisa ter o Rust instalado. Você pode instalar o Rust usando [rustup](https://rustup.rs/).

Clone o repositório e compile o projeto:

## Como usar

~~~bash
rustsync -s <Diretorio/de/origem> -d <Diretorio/de/destino> -v <Imprime o que esta acontecendo>
~~~

### Informações de argumentos

1. (-s) Diretorio de origem a ser feito backup.
2. (-d) Diretorio de destino do backup.
3. (-v) Ativa o modo verbose para exibir mensagem.


### Comentários no Código

O código original com comentario para auxiliar:

```rust
use clap::Parser;
use dotenv::dotenv;
use std::fs::File;
use std::io::{self};
use tar::Builder;
use walkdir::WalkDir;

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

    /// Modo verbose
    #[arg(short = 'v', long)]
    verbose: bool,
}

/// Função que realiza o backup do diretório de origem para o destino
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
```