//! Portfolio application complete with blog and accounts.

/// Database connection and communication utilies and schemas.
pub mod database;
/// Route access handling.
pub mod routing;
/// Various general utilies that abstract boilerplate.
pub mod util;

/// Suprisingly this is the entry point of the application ヽ(°〇°)ﾉ!!!
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util::setup::init_app().await?;

    Ok(())
}
