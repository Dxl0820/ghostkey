use crate::error::Result;
use crate::vault::Vault;

pub async fn execute(port: Option<u16>, no_open: bool) -> Result<()> {
    // Check vault exists (don't unlock yet - user unlocks via Web UI)
    let vault = Vault::open()?;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port.unwrap_or(3001)));

    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| crate::error::Error::ConfigError(format!("Failed to bind: {}", e)))?;

    let actual_addr = listener.local_addr()
        .map_err(|e| crate::error::Error::ConfigError(format!("Failed to get address: {}", e)))?;

    let url = format!("http://{}", actual_addr);

    println!("GhostKey API running at {}", url);
    println!("Press Ctrl+C to stop.");
    println!();
    println!("Open the Web UI and unlock with your master password.");

    if !no_open {
        if let Err(e) = open::that(&url) {
            println!("Could not open browser: {}", e);
            println!("Please open {} manually", url);
        }
    }

    // Start the API server (vault starts locked, user unlocks via API)
    let state = std::sync::Arc::new(tokio::sync::Mutex::new(vault));
    let app = crate::api::routes::create_router(state);

    axum::serve(listener, app).await
        .map_err(|e| crate::error::Error::ConfigError(format!("Server error: {}", e)))?;

    Ok(())
}
