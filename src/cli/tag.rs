use crate::error::Result;
use crate::vault::Vault;

pub fn list_tags() -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let tags = vault.list_tags()?;

    if tags.is_empty() {
        println!("No tags found");
        println!("  Add tags when creating credentials: ghostkey add <name>");
        return Ok(());
    }

    println!("Tags:");
    println!();

    for (tag, count) in tags {
        println!("  {} ({} credential(s))", tag, count);
    }

    Ok(())
}

pub fn add_tag(credential: &str, tag: &str) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    vault.add_tag(credential, tag)?;

    println!("✓ Tag '{}' added to '{}'", tag, credential);

    Ok(())
}

pub fn remove_tag(credential: &str, tag: &str) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    vault.remove_tag(credential, tag)?;

    println!("✓ Tag '{}' removed from '{}'", tag, credential);

    Ok(())
}
