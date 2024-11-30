use std::fs;
use std::path::Path;
use chrono::Local;

pub fn create_backup_structure(destination: &str, backup_name: &str) -> std::io::Result<()> {
    // Obter a data atual
    let now = Local::now();
    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    let day = now.format("%d").to_string();

    // Construir o caminho completo
    let path = Path::new(destination)
        .join(backup_name)
        .join(year)
        .join(month)
        .join(day);

    // Criar a estrutura de diretórios
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}