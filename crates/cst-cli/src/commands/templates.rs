use anyhow::Result;

pub fn list() -> Result<()> {
    for t in cst_core::templates::all() {
        println!("{:<12} [{}]  {}", t.name, t.auth_type, t.description);
    }
    Ok(())
}
