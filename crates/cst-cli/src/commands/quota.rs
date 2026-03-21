use anyhow::Result;
pub fn remaining() -> Result<()> {
    println!("Quota tracking requires daemon. Run: cst daemon start");
    Ok(())
}
