use serde::Deserialize;

#[derive(Deserialize)]
pub struct BackupConfig {
    pub source: String,
    pub destination: String,
    pub backup_name: String,
    pub cron: String,
    pub verbose: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub backups: Vec<BackupConfig>,
}

pub fn load_config(file_path: &str) -> Config {
    let file = std::fs::File::open(file_path).expect("Erro ao abrir o arquivo de configuração");
    serde_yaml::from_reader(file).expect("Erro ao parsear o arquivo de configuração")
}