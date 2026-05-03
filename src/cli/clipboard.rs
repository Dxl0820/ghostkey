use crate::error::Result;
use crate::vault::Vault;
use std::time::Duration;

pub fn execute(name: &str, timeout: Option<u64>) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let secret = vault.get_secret(name)?;

    // Copy to clipboard
    crate::utils::clipboard::copy_to_clipboard(&secret)?;

    let timeout_secs = timeout.unwrap_or(30);

    println!("✓ Credential '{}' copied to clipboard", name);
    println!("  Clipboard will be cleared in {} seconds", timeout_secs);
    println!("  Press Ctrl+C to keep in clipboard");

    // Spawn a thread to clear clipboard after timeout
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(timeout_secs));

        // Clear clipboard by copying empty string
        if let Err(e) = clear_clipboard() {
            eprintln!("Warning: Failed to clear clipboard: {}", e);
        } else {
            // Print message (may not work in all terminals)
            eprintln!("✓ Clipboard cleared");
        }
    });

    // Wait for user input or timeout
    println!("  Press Enter to clear clipboard early...");

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        tx.send(()).ok();
    });

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(_) => {
            clear_clipboard()?;
            println!("✓ Clipboard cleared");
        }
        Err(_) => {
            // Timeout reached, clipboard already cleared by thread
        }
    }

    Ok(())
}

fn clear_clipboard() -> Result<()> {
    crate::utils::clipboard::copy_to_clipboard("")
}

pub fn copy_with_timeout(text: &str, timeout_secs: u64) -> Result<()> {
    crate::utils::clipboard::copy_to_clipboard(text)?;

    println!("✓ Copied to clipboard");
    println!("  Will be cleared in {} seconds", timeout_secs);

    // Spawn cleanup thread
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(timeout_secs));
        let _ = clear_clipboard();
    });

    Ok(())
}
