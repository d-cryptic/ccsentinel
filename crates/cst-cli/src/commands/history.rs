use anyhow::Result;
pub fn run() -> Result<()> {
    println!("No switch history yet.");
    Ok(())
}
pub fn why() -> Result<()> {
    let cfg = cst_core::GlobalConfig::load()?;
    if cfg.current_profile.is_empty() {
        println!("No active profile.");
    } else {
        println!("Active: {} (manually activated)", cfg.current_ref());
    }
    Ok(())
}
