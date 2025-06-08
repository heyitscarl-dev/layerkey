use common::err::Result;
use log::LevelFilter;

mod common;

fn main() -> Result<()> {
    common::log::init(LevelFilter::Trace);

    Ok(())
}
