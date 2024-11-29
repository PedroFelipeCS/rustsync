use env_logger::Builder;
use env_logger::Env;
use std::io::Write;
use std::time::SystemTime;

pub fn init_logger() {
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
}
