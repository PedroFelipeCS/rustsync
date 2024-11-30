use std::fs;
use std::path::Path;
use chrono::Local;

/// Cria a estrutura de diretórios para o backup.
///
/// A estrutura de diretórios será criada no formato:
/// `<destination>/<backup_name>/<ano>/<mes>/<dia>/`.
///
/// # Argumentos
///
/// * `destination` - Diretório de destino.
/// * `backup_name` - Nome do backup.
///
/// # Erros
///
/// Retorna um erro se ocorrer algum problema durante a criação dos diretórios.
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