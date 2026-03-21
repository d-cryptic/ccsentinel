use anyhow::Result;

pub async fn dispatch(action: crate::AutoSwitchCommands) -> Result<()> {
    match action {
        crate::AutoSwitchCommands::Configure { profile } => configure(&profile),
        crate::AutoSwitchCommands::Log => log(),
        crate::AutoSwitchCommands::Test { profile } => test_chain(&profile),
    }
}
pub fn configure(profile: &str) -> Result<()> {
    println!("Auto-switch configure for '{profile}' — not yet implemented");
    Ok(())
}
pub fn log() -> Result<()> { println!("No auto-switch events yet"); Ok(()) }
pub fn test_chain(profile: &str) -> Result<()> {
    println!("Dry-run for '{profile}' — not yet implemented");
    Ok(())
}
pub fn pause(minutes: Option<u64>) -> Result<()> {
    let m = minutes.unwrap_or(30);
    println!("Auto-switch paused for {m} minutes");
    Ok(())
}
