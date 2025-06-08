use log::LevelFilter;

pub fn init(filter: LevelFilter) {
    env_logger::builder()
        .filter_level(filter)
        .init();

    log::info!("initialised logs")
}
