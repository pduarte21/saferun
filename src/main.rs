use std::env;
use std::process::Command;
use std::fs;
use tempfile::tempdir;

enum CommandType {
    Run { script: String },
}

fn parse_args() -> CommandType {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("run") => {
            let script = args.next().expect("missing script");
            CommandType::Run { script }
        }
        _ => {
            panic!("usage: saferun run <script>");
        }
    }
}

fn main() {
    let cmd = parse_args();

    match cmd {
        CommandType::Run { script } => {
            println!("[saferun] Running script: {}", script);

            println!("[saferun] Isolation
    - filesystem: temporary
    - network: restricted (basic)
    - environment: clean");

            let contents = fs::read_to_string(&script)
                .expect("failed to read script");

            if let Some(tool) = contains_blocked_network_tools(&contents) {
                eprintln!("[saferun] blocked: script uses forbidden network tool '{}'", tool);
                std::process::exit(1);
            }

            let dir = tempdir().expect("failed to create temp dir");
            let temp_path = dir.path();

            println!("[saferun] Using temp dir: {:?}", temp_path);

            let script_name = std::path::Path::new(&script)
                .file_name()
                .expect("invalid script path");

            let temp_script_path = temp_path.join(script_name);

            fs::copy(&script, &temp_script_path)
                .expect("failed to execute script");

            let output = Command::new("sh")
                .arg(&script_name)
                .current_dir(temp_path)
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .env("HOME", temp_path)
                .output()
                .expect("failed to execute script");

            println!("[saferun] Output:");
            
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

fn contains_blocked_network_tools(script_contents: &str) -> Option<&'static str> {
    let blocked = ["curl", "wget", "nc", "netcat", "scp"];

    for tool in blocked {
        if script_contents.contains(tool) {
            return Some(tool);
        }
    }

    None
}