use anyhow::Result;

pub async fn dispatch(action: crate::DaemonCommands) -> Result<()> {
    match action {
        crate::DaemonCommands::Start => start().await,
        crate::DaemonCommands::Stop => stop(),
        crate::DaemonCommands::Restart => { stop()?; start().await }
        crate::DaemonCommands::Status => status(),
        crate::DaemonCommands::Logs => logs(),
    }
}

pub async fn start() -> Result<()> {
    println!("Daemon start — not yet implemented (coming in task #5)");
    Ok(())
}
pub fn stop() -> Result<()> { println!("Daemon stop — not yet implemented"); Ok(()) }
pub fn status() -> Result<()> { println!("Daemon: not running"); Ok(()) }
pub fn logs() -> Result<()> { println!("No daemon logs yet"); Ok(()) }
