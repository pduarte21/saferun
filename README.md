# saferun

⚠️ Stop blindly running scripts.

**See what a shell script will do before you run it.**

## Example

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

## The problem
We've all done this:
```bash
curl something.sh | bash
```

You don't really know what that script will do.

It might:
- modify your files
- leak environment variables
- access unexpected parts of your system
- execute other scripts

## The solution
```saferun``` helps you **quickly understand what a script might do before you run it**.

It adds a lightweight safety layer to your normal workflow:
- inspect scripts with the `--dry-run`
- detect risky patterns
- highlight file system impact
- warn before execution when needed

## How it works
### Preview mode (dry-run)
```
saferun run script.sh --dry-run
```

- shows risk level
- explains suspicious patterns
- highlights file operations
- detects network usage

### Smart execution
`saferun` adapts its behaviour based on risk level:
- **LOW risk** -> runs quietly (no interruptions)
- **MEDIUM/HIGH risk** -> shows warnings and asks before executing

### Example (safe script)
```
./saferun run examples/safe_script.sh
```

```
Hello world
This is a safe script
```

### Safety Prompt
If a script looks suspicious:
```
[saferun] Risk: HIGH 🚨

[warning] Potential risks detected:
 - rm -rf
 - curl

Continue? (y/N):
```

## Advanced usage
### JSON output
You can get structured output for automation:
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
- scripting and automation
- CI/CD pipelines
- integrating with other tools

## Lightweight isolation
When running scripts, `saferun` applies basic protections:
- temporary working directory
- clean environment (`env_clear`)
- minimal `PATH`
- isolated `HOME`
- basic blocking of network tools

## Install

### Download binary
Go to the [Releases page](https://github.com/pduarte21/saferun/releases) and download the binary for your OS.

#### Linux (x86_64)
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-linux-x86_64
chmod +x saferun-linux-x86_64
mv saferun-linux-x86_64 saferun
```

#### macOS (Apple Silicon)
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-macos-aarch64
chmod +x saferun-macos-aarch64
mv saferun-macos-aarch64 saferun
```

#### Windows
Download `saferun-windows-x86_64.exe` and run it from PowerShell or CMD.

## Examples
Try the included scripts:
```
./saferun run examples/suspicious-install.sh --dry-run
./saferun run examples/suspicious-install.sh
```

## What saferun protect againts
- blindly running unknown scripts
- accidental file modifications
- leaking environment variables
- obvious unsafe patterns (e.g. `rm -rf`, `curl`, `chmod 777`)
- basic network calls

## Limitations
This is **not a full sandbox**.

It does NOT prevent:
- access via absolute paths
- dynamic or obfuscated behaviour
- indirect execution
- real OS-level isolation

## Why not just use a VM
VMs and container provide stronger isolation, but:
- they add friction
- they are not used for quick, everyday scripts

`saferun` is designed to be:
> a fast, low-friction safety layer for the common case

## Status
Early prototype focused on:
- usability
- visibility
- real-world developer workflows

Feedback is very welcome.