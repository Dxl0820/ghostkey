#!/bin/bash

# GhostKey Usage Examples

echo "=== GhostKey Usage Examples ==="
echo ""

# Initialize vault
echo "1. Initialize vault:"
echo "   ghostkey init"
echo ""

# Add credentials
echo "2. Add credentials:"
echo "   ghostkey add github-token --type apikey --username myuser"
echo "   ghostkey add aws-key --type apikey --tags cloud,aws"
echo "   ghostkey add database-url --type env"
echo "   ghostkey add ssh-key --type ssh"
echo ""

# List credentials
echo "3. List credentials:"
echo "   ghostkey list"
echo "   ghostkey list --tag cloud"
echo "   ghostkey list --type apikey"
echo "   ghostkey list --names-only"
echo ""

# Get credentials
echo "4. Get credentials:"
echo "   ghostkey get github-token"
echo "   ghostkey get github-token --show"
echo "   ghostkey get github-token --clipboard"
echo "   ghostkey get github-token --json"
echo ""

# Manage tags
echo "5. Manage tags:"
echo "   ghostkey tag list"
echo "   ghostkey tag add github-token production"
echo "   ghostkey tag remove github-token development"
echo ""

# Export/Import
echo "6. Export/Import:"
echo "   ghostkey export --format env > .env"
echo "   ghostkey export --format json > backup.json"
echo "   ghostkey import --format env .env"
echo "   ghostkey import --format json backup.json"
echo ""

# Scripting examples
echo "7. Scripting examples:"
echo '   SECRET=$(ghostkey get github-token --json --show | jq -r ".secret")'
echo '   ghostkey list --names-only | while read name; do echo "Processing: $name"; done'
echo ""

echo "For more information, run: ghostkey --help"
