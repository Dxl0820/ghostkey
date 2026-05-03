# GhostKey Advanced Usage

## SSH Key Management

### Adding SSH Keys

```bash
# Add an SSH key
ghostkey ssh add my-server-key

# Interactive prompts will ask for:
# - SSH key path (e.g., ~/.ssh/id_ed25519)
# - Username
# - Host/URL
# - Description
# - Tags
# - Port (optional)
# - ProxyJump (optional)
# - ForwardAgent (optional)
```

### Listing SSH Keys

```bash
# List all SSH keys
ghostkey ssh list
```

### Generating SSH Config

```bash
# Generate SSH config for ~/.ssh/config
ghostkey ssh config

# Save to file
ghostkey ssh config > ~/.ssh/config.d/ghostkey
```

## Environment Variables

### Exporting Environment Variables

```bash
# Export all env vars (bash/zsh)
ghostkey env export

# Export for fish shell
ghostkey env export --shell fish

# Export for PowerShell
ghostkey env export --shell powershell

# Save to file
ghostkey env export > .env

# Source directly
source <(ghostkey env export)
```

### Setting Individual Variables

```bash
# Set a single env var
ghostkey env set database-url

# Set for specific shell
ghostkey env set database-url --shell fish
```

### Unsetting Variables

```bash
# Unset an env var
ghostkey env unset database-url
```

## Clipboard with Auto-Clear

### Copy with Timeout

```bash
# Copy with 30-second timeout (default)
ghostkey clipboard github-token

# Copy with custom timeout
ghostkey clipboard github-token --timeout 60
```

### Using in Scripts

```bash
# Copy and wait
ghostkey clipboard github-token --timeout 10
# Clipboard will be cleared after 10 seconds
```

## Search

### Basic Search

```bash
# Search by name, description, tags, etc.
ghostkey search github

# Search with JSON output
ghostkey search aws --json
```

### Search Patterns

```bash
# Search for API keys
ghostkey search api

# Search for cloud credentials
ghostkey search cloud

# Search for specific user
ghostkey search myuser
```

## Shell Completions

### Generate Completions

```bash
# Bash
ghostkey completion bash > /etc/bash_completion.d/ghostkey

# Zsh
ghostkey completion zsh > ~/.zfunc/_ghostkey

# Fish
ghostkey completion fish > ~/.config/fish/completions/ghostkey.fish

# PowerShell
ghostkey completion powershell > ghostkey.ps1
```

### Installing Completions

#### Bash

```bash
# Add to ~/.bashrc
eval "$(ghostkey completion bash)"
```

#### Zsh

```bash
# Add to ~/.zshrc
autoload -Uz compinit
compinit
eval "$(ghostkey completion zsh)"
```

#### Fish

```bash
# Fish completions are automatic
ghostkey completion fish > ~/.config/fish/completions/ghostkey.fish
```

#### PowerShell

```powershell
# Add to $PROFILE
Invoke-Expression -Command $(ghostkey completion powershell | Out-String)
```

## JSON Output

### Using JSON in Scripts

```bash
# Get credential as JSON
ghostkey get github-token --json

# Extract secret with jq
SECRET=$(ghostkey get github-token --json --show | jq -r '.secret')

# List all credentials as JSON
ghostkey list --json

# Search and process
ghostkey search api --json | jq '.[] | .name'
```

### JSON Structure

```json
{
  "name": "github-token",
  "type": "API Key",
  "username": "myuser",
  "description": "GitHub personal access token",
  "tags": ["github", "development"],
  "url": "https://github.com",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

## Import/Export

### Export Formats

#### Environment Variables (.env)

```bash
ghostkey export --format env > .env
```

Output:
```
GITHUB_TOKEN=ghp_xxxxxxxxxxxx
AWS_KEY=AKIAIOSFODNN7EXAMPLE
```

#### JSON

```bash
ghostkey export --format json > backup.json
```

#### CSV

```bash
ghostkey export --format csv > credentials.csv
```

### Import Formats

#### From .env File

```bash
ghostkey import --format env .env
```

#### From JSON

```bash
ghostkey import --format json backup.json
```

#### From CSV

```bash
ghostkey import --format csv credentials.csv
```

## Batch Operations

### Export All Credentials

```bash
# Export everything
ghostkey export --format json > all-credentials.json

# Export by tag
ghostkey export --format env --tag production > prod.env
```

### Import from Other Tools

```bash
# Import from 1Password CSV
ghostkey import --format csv 1password-export.csv

# Import from LastPass
ghostkey import --format csv lastpass-export.csv
```

## Integration with Other Tools

### Using with direnv

```bash
# .envrc
eval "$(ghostkey env export)"
```

### Using with Docker

```dockerfile
# Dockerfile
RUN ghostkey env export > /app/.env
```

### Using with CI/CD

```yaml
# GitHub Actions
- name: Load secrets
  run: |
    eval "$(ghostkey env export)"
    echo "::add-mask::$API_KEY"
```

## Security Best Practices

### Clipboard Security

- Use `ghostkey clipboard` instead of `ghostkey get --clipboard`
- Set appropriate timeout (default: 30 seconds)
- Don't use in shared terminals

### Script Security

- Use `--json` output for parsing
- Don't log secrets
- Use environment variables when possible

### Export Security

- Export to encrypted storage
- Delete export files after use
- Use `--tag` to limit exports

## Troubleshooting

### Common Issues

#### Clipboard not working

```bash
# Check if clipboard tools are installed
# Linux: xclip or xsel
# macOS: pbcopy (built-in)
# Windows: clip (built-in)
```

#### Shell completions not working

```bash
# Regenerate completions
ghostkey completion bash > ~/.local/share/bash-completion/completions/ghostkey

# Restart shell
exec bash
```

#### Import fails

```bash
# Check file format
cat import-file.csv

# Try with verbose output
ghostkey import --format csv import-file.csv
```
