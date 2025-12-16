function Show-Progress {
    param (
        [string]$Activity,
        [int]$Seconds
    )
    for ($i = 0; $i -le 100; $i += 10) {
        Write-Progress -Activity $Activity -Status "$i% Complete" -PercentComplete $i
        Start-Sleep -Milliseconds ($Seconds * 100)
    }
    Write-Progress -Activity $Activity -Completed
}

$rustupPath = "$env:USERPROFILE\.cargo\bin\rustup.exe"

if (Test-Path $rustupPath) {
    Write-Host "✅ rustup is already installed!" -ForegroundColor Green
} else {
    Write-Host "❌ rustup not found..." -ForegroundColor Yellow
    Write-Host "🚀 Installing rustup, please wait..." -ForegroundColor Cyan

    # Show a fake progress bar while downloading
    Show-Progress -Activity "Downloading rustup" -Seconds 0.2

    # Show another progress bar while installing
    Show-Progress -Activity "Installing rustup" -Seconds 0.3
    
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "C:\Users\$env:USERNAME\Downloads\rustup-init.exe"
    Start-Process "C:\Users\$env:USERNAME\Downloads\rustup-init.exe" -ArgumentList "-y" -NoNewWindow -Wait

    <#  
        --- Install the Following tools in Visula Studio under Individual Components ---
            MSVC v143 - VS 2022 C++ x64/x86 build tools
            Windows 11 SDK (10.0.22621.0)
    #>

    Write-Host "🎉 Installed successfully!" -ForegroundColor Green
    Write-Host "Run: rust-cli --help" -ForegroundColor Magenta

    Write-Host " 
        !!! Install the Following tools in Visula Studio under Individual Components !!!

            - MSVC v143 - VS 2022 C++ x64/x86 build tools
            - Windows 11 SDK (10.0.22621.0)

            If not installed you will run into Compile issues on Windows!
        " -ForegroundColor Red
}