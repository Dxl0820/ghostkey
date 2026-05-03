use std::io::{self, Write};
use crate::error::Result;

pub fn prompt(message: &str) -> Result<String> {
    print!("{}: ", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn prompt_optional(message: &str) -> Result<Option<String>> {
    let input = prompt(message)?;
    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

pub fn prompt_password(message: &str) -> Result<String> {
    let password = rpassword::prompt_password(message)?;
    Ok(password.to_string())
}

pub fn confirm(message: &str) -> Result<bool> {
    print!("{} [y/N]: ", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}
