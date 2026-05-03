use crate::error::Result;
use crate::vault::Vault;

pub fn execute(name: &str, force: bool) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    // Verify credential exists
    let _credential = vault.get_credential(name)?;

    if !force {
        print!("Delete credential '{}'? [y/N]: ", name);
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled");
            return Ok(());
        }
    }

    vault.delete_credential(name)?;

    println!("✓ Credential '{}' deleted", name);

    Ok(())
}
