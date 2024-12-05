# Rustsync

Rustsync é um sistema de backup de diretório escrito em Rust. Ele cria um arquivo tar contendo todos os arquivos e diretórios do diretório de origem especificado.

## Instalação

Para compilar e instalar o `rustsync`, você precisa ter o Rust instalado. Você pode instalar o Rust usando [rustup](https://rustup.rs/).

Clone o repositório e compile o projeto:
~~~sh
git clone <URL_DO_REPOSITORIO>
cd rustsync
cargo build --release
~~~

## Como usar

### Usando Argumentos de Linha de Comando

~~~bash
rustsync -s <Diretorio/de/origem> -d <Diretorio/de/destino> -n <Nome do backup> -v <Imprime o que está acontecendo>
~~~

### Usando um Arquivo de Configuração

Você pode usar um arquivo de configuração YAML para definir múltiplos backups. Aqui está um exemplo de como usar o `rustsync` com um arquivo de configuração:

~~~bash
rustsync -c config.yaml
~~~

### Exemplo de Arquivo de Configuração (config.yaml)

```yaml
backups:
  - source: "/path/to/source1"
    destination: "/path/to/destination1"
    backup_name: "backup1"
    cron: "0 0 * * *"  # Todos os dias à meia-noite
    verbose: true
  - source: "/path/to/source2"
    destination: "/path/to/destination2"
    backup_name: "backup2"
    cron: "0 12 * * *"  # Todos os dias ao meio-dia
    verbose: false
```

### Informações de argumentos

1. (-s) Diretório de origem a ser feito backup.
2. (-d) Diretório de destino do backup.
3. (-n) Nome do arquivo de backup.
4. (-c) Caminho para o arquivo de configuração YAML.
5. (-r) Expressão para cron agendamento.
6. (-v) Ativa o modo verbose para exibir mensagens detalhadas.

### Configurando como um serviço systemd

Você pode configurar o rustsync para ser executado automaticamente usando o systemd. Aqui está um exemplo de como fazer isso:

#### Criar uma arquivo de unidade systemd

Crie um arquivo de unidade systemd em `/etc/systemd/system/rustsync.service` com o seguinte conteúdo.

~~~bash
[Unit]
Description=Rustsync Backup Service
After=network.target

[Service]
ExecStart=/caminho/para/seu/programa/rustsync -c /caminho/para/config.yaml
Restart=always
User=seu_usuario
Group=seu_grupo

[Install]
WantedBy=multi-user.target
~~~

#### Recarregar o systemd e habilitar o serviço

Depois de cirar o arquivo de unidade, recarregue o `systemd` para reconhecer o novo serviço e, em seguida, habilitar o inicie o serviço

~~~bash
sudo systemctl daemon-reload
sudo systemctl enable rustsync.service
sudo systemctl start rustsync.service
~~~

#### Reinciar o serviço para aplicar novas configurações

Para aplica novas configurações ao `Rustsync` você precisa reinciar o serviço utilize:

~~~bash
systemctl restart rustsync.service
~~~

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
````