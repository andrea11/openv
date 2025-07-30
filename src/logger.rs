use env_logger::Builder;
use log::LevelFilter;

pub fn init_logger(verbose: u8) {
    let mut log_builder = Builder::from_env(env_logger::Env::default());

    let level = match verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log_builder.filter_level(level);
    log_builder.init();
}
