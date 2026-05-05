# saferun

⚠️ Stop blindly running scripts.

**See what a shell script might do before you run it.**

## Quick Start

```
./saferun run examples/suspicious-install.sh --dry-run
```

```
[saferun] Preview mode (best-effort analysis)
----------------------------------------

[saferun] Risk: HIGH 🚨

[warning] Potential risks detected:

[HIGH]
 - rm -rf -> deletes files recursively (can wipe directories)

[MEDIUM]
 - chmod 777 -> makes files globally writable

[INFO]
 - curl -> downloads remote content

[info] File operations:
 - /tmp/* (delete)

----------------------------------------
[saferun] No changes were made.
```

No changes are made. Just insight.

## Why this exists
We've all done this:
```bash
curl something.sh | bash
```

Fast, convenient... and risky.

You don't really know what that script will do.

It might:
- modify your files
- leak environment variables
- download and execute other code
- touch unexpected parts of your system

## What saferun does
```saferun``` helps you **understand risk before execution**, without slowing you down.

It adds a lightweight safety layer to your workflow:
- preview scripts (`--dry-run`)
- detect risky patterns
- highlight file system impact
- warn before execution when needed

## When should I use saferun?
Use `saferun` when you're about to run a script you don't fully trust.

### Typical cases
- Running scripts from the internet
- Trying install scripts from GitHub
- Copy-pasting commands from blogs or StackOverflow
- Running quick fixes you don't fully understand

## Preview mode (dry-run)
```
./saferun run script.sh --dry-run
```

- shows risk level
- explains suspicious patterns
- detects network usage
- highlights file operations

## Smart execution
When you run a script:
```
./saferun run script.sh
```
- **LOW risk** - runs normally
- **MEDIUM/HIGH risk** - shows warnings and asks for confirmation 

```
[saferun] Risk: HIGH 🚨

[warning] Potential risks detected:
 - rm -rf
 - curl

Continue? (y/N): 
```

## Examples
Try the included scripts:
```
./saferun run examples/hello.sh --dry-run
./saferun run examples/network_attempt.sh --dry-run
./saferun run examples/suspicious_installs.sh
```

## Windows usage
`saferun` supports:
- `.bat` (via `cmd`)
- `.ps1` (via PowerShell)

Examples:
```
./saferun run examples/hello.ps1 --dry-run
./saferun run examples/hello.bat --dry-run
```
Notes: 
- `.ps1` requires PowerShell (`pwsh` or `powershell`)
- `.bat` is supported on Windows only
- On macOS/Linux, Windows scripts can be analyzed but are not executed by default

## JSON output
For automation:
```
./saferun run script.sh --dry-run --json
```

```
{
  "risk_level": "HIGH",
  "command_count": 8,
  "warnings": [
    {
      "pattern": "rm -rf",
      "severity": "HIGH",
      "explanation": "deletes files recursively (can wipe directories)"
    }
  ],
  "network_usage": true,
  "file_operations": [
    {
      "path": "/tmp/file.txt",
      "operation": "write"
    }
  ]
}
```

Useful for:
- CI/CD pipelines
- scripting
- integrations

## Lightweight isolation
When running scripts, `saferun` applies basic protections:
- temporary working directory
- clean environment (`env_clear`)
- minimal `PATH`
- isolated `HOME`

## Limitations
This is **not a full sandbox**.

It does NOT guarantee:
- full filesystem isolation
- protection against obfuscated or dynamic behaviour
- blocking of all network access
- OS-level security guarantees

## Why not just use a VM
VMs and container provide stronger isolation, but:
- they add friction
- they are not used for quick, everyday scripts

`saferun` is designed to be:
> a fast, low-friction safety layer for everyday scripts


## Install

Go to the [Releases page](https://github.com/pduarte21/saferun/releases) and download the binary for your OS.

### Linux (x86_64)
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-linux-x86_64
chmod +x saferun-linux-x86_64
mv saferun-linux-x86_64 saferun
```

### macOS (Apple Silicon)
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-macos-aarch64
chmod +x saferun-macos-aarch64
mv saferun-macos-aarch64 saferun
```

### Windows
Download `saferun-windows-x86_64.exe` and run:
```
saferun-windows-x86_64.exe run script.ps1 
```

## Status
Early-stage project focused on:
- usability
- real developer workflows
- fast feedback loops

## Feedback
Feedback, issues and ideas are very welcome.