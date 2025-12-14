$BinName = "rust-cli.exe"
$InstallDir = "$env:USERPROFILE\.cargo\bin"

Write-Host "Building rust-cli..."
cargo build --release

Write-Host "Installing to $InstallDir..."
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

Copy-Item "target\release\$BinName" "$InstallDir\$BinName" -Force

Write-Host "✅ Installed successfully"
Write-Host "Run: rust-cli --help"