use crate::error::Result;

#[cfg(target_os = "windows")]
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    use clipboard_win::{Clipboard, Setter, formats::Unicode};
    let _clipboard = Clipboard::new()
        .map_err(|e| crate::error::Error::Other(format!("Clipboard error: {:?}", e)))?;
    Unicode.write_clipboard(&text.to_string())
        .map_err(|e| crate::error::Error::Other(format!("Clipboard write error: {:?}", e)))?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    use std::io::Write;
    use std::process::Command;
    let mut child = Command::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(text.as_bytes())?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    use std::io::Write;
    use std::process::Command;
    let mut child = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(text.as_bytes())?;
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn copy_to_clipboard(_text: &str) -> Result<()> {
    Err(crate::error::Error::Other(
        "Clipboard not supported on this platform".to_string()
    ))
}
