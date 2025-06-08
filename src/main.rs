use common::error::Result;
use lexer::tokenise;
use log::LevelFilter;

mod common;
mod lexer;

fn main() -> Result<()> {
    common::log::init(LevelFilter::Trace);
    let source = String::from(r#"
    !setProcessAll true
    "#);

    let tokens = tokenise(source)?;

    for token in tokens {
        token.flag().log();
    }

    Ok(())
}
