# saferun

⚠️ Running random scripts? Don’t trust them blindly.

Run untrusted scripts safely — no setup, no surprises.

---

## The problem
We've all done this:
```bash
curl something.sh | bash
```

Sometimes it works.

Sometimes... you have no idea what just ran on your system.

## The solution
```saferun``` lets you run any script in a strongly isolated environment:
- No access to your filesystem
- No network access (by default)
- Restricted system calls
- Clean environment every time

## Quick start
```
saferun run script.sh
```
That's it.

## What actually happens
When you run a script with ```saferun```:
- It runs in a isolated filesystem
- It cannot access your files
- It cannot reach the network
- It runs with limited system capabilities

## Example
```
saferun run install.sh
```

Output:
```
[saferun] Running in isolated environment
[saferun] Filesystem: isolated
[saferun] Network: blocked
[saferun] Execution started...

(output of the script)

[saferun] Execution finished
```

## Why not Docker?
Docker is powerful, but:
- Requires setup
- Not designed for adversarial scripts
- Easy to misconfigure

```saferun``` is:
- Minimal
- Focused
- Safe by default

## When to use this
- Running scripts from the internet
- Testing unknown code
- Inspecting install scripts
- Anything you don't fullt trust

## What this is (and isn't)
This is:
- A simple tool for safer local execution

This is not:
- A full VM
- A CI system
- A replacement for proper security practices

## Examples

You can try saferun with the provided scripts:

- `examples/hello.sh` — basic execution
- `examples/env.sh` — environment isolation
- `examples/evil.sh` — simulates unsafe behavior

## Limitations

This version does not provide full filesystem or network isolation.

## Status
Early version, focused on simplicity and safety.

