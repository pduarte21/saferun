use std::{env};
use std::process::Command;
use std::fs;
use tempfile::tempdir;
use std::io::{self, Write};

enum CommandType {
    Run { script: String, dry_run: bool },
}

#[derive(Clone)]
struct Warning {
    pattern: String,
}

struct AnalysisResult {
    command_count: usize,
    warnings: Vec<Warning>,
    network_usage: bool,
    file_ops: Vec<String>,
    risk_level: String,
}

fn parse_args() -> CommandType {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("run") => {
            let script = args.next().expect("missing script");
            let dry_run = args.any(|arg| arg == "--dry-run");

            CommandType::Run { script, dry_run }
        }
        _ => {
            panic!("usage: saferun run <script> [--dry-run]");
        }
    }
}

fn analyze_script(contents: &str) -> AnalysisResult {
    let mut warnings = Vec::new();
    let mut file_ops = Vec::new();
    let mut network_usage = false;
    let mut score = 0;

    let dangerous_patterns = ["rm -rf", "chmod 777", "dd", "mkfs"];
    let network_patterns = ["curl", "wget", "nc", "ssh"];
    let sensitive_paths = ["/etc/passwd", "/etc/shadow", ".ssh", ".aws"];

    let lines: Vec<&str> = contents.lines().collect();

    for line in &lines {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        for pattern in &dangerous_patterns {
            if line.contains(pattern) {
                warnings.push(Warning {
                    pattern: pattern.to_string()
                });
            }
        }

        for pattern in &network_patterns {
            if line.contains(pattern) {
                network_usage = true;
                warnings.push(Warning {
                    pattern: pattern.to_string(),
                });
            }
        }

        if line.contains(">") || line.contains(">>") {
            file_ops.push(line.to_string());
        }
        if line.contains("rm ") || line.contains("cp ") || line.contains("mv ") {
            file_ops.push(line.to_string());
        }
        if line.contains("| bash") {
            warnings.push(Warning {
                pattern: "piped execution".to_string()
            });
        }

        for path in &sensitive_paths {
            if line.contains(path) {
                warnings.push(Warning { 
                    pattern:  format!("sensitive path access: {}", path),
                });
            }
        }
    }

    score += warnings.len();
    if network_usage {
        score += 2;
    }
    if file_ops.len() > 3 {
        score += 1;
    }

    let risk_level = match score {
        0     => "LOW",
        1..=3 => "MEDIUM",
        _     => "HIGH",
    }.to_string();

    warnings.sort_by_key(|w| {
        std::cmp::Reverse(severity_weight(&w.pattern))
    });

    let command_count = lines.iter()
    .filter(|line| {
        let l = line.trim();
        !l.is_empty() && !l.starts_with('#')
    })
    .count();

    AnalysisResult { command_count, warnings, network_usage, file_ops, risk_level }
}

fn explain_pattern(pattern: &str) -> &'static str {
    match pattern {
        "rm -rf" => "deletes files recursively (can wipe directories)",
        "chmod 777" => "makes files globally writable",
        "dd " => "low-level disk write (can corrupt data)",
        "mkfs" => "formats a filesystem (destructive)",
        "curl" | "wget" => "downloads remote content",
        "nc " | "netcat" => "opens raw network connections",
        "ssh" => "connects to remote machines",
        "piped execution" => "executes piped or downloaded content",
        _ => "potentially unsafe operation",
    }
}

fn severity(pattern: &str) -> &'static str {
    match pattern {
        "rm -rf" | "dd " | "mkfs" => "HIGH",
        "chmod 777" => "MEDIUM",
        "curl" | "wget" | "nc " | "ssh" => "INFO",
        "piped execution" => "HIGH",
        _ => "INFO", 
    }
}

fn severity_weight(pattern: &str) -> u8 {
    match severity(pattern) {
        "HIGH" => 3,
        "MEDIUM" => 2,
        "INFO" => 1,
        _ => 0,
    }
}

fn main() {
    let cmd = parse_args();

    match cmd {
        CommandType::Run { script, dry_run } => {
            let contents = fs::read_to_string(&script)
            .expect("failed to read script");

            let analysis = analyze_script(&contents);

            if dry_run {
                println!("[saferun] ⚠️  This is a best-effort preview. Bash scripts can be dynamic.\n");
                println!("[saferun] Dry run mode (no execution)\n");
                println!("[saferun] Risk Level: {}\n", analysis.risk_level);

                println!("[info] Script summary:");
                println!(" - commands: {}", analysis.command_count);

                if !analysis.warnings.is_empty() {
                    println!("\n[warning] Potentially dangerous patterns:");
                    for w in &analysis.warnings {
                        let pattern = &w.pattern;
                        let sev = if pattern.starts_with("sensitive path") {
                            "HIGH"
                        } else {
                            severity(pattern)
                        };
                        let explanation = if pattern.starts_with("sensitive path") {
                            "accesses potentially sensitive system data"   
                        } else {
                            explain_pattern(pattern)
                        };

                        println!(" - [{}] {} -> {}", sev, pattern, explanation);
                    }
                }

                if analysis.network_usage {
                    println!("\n[info] Network access: detected");
                }

                if !analysis.file_ops.is_empty() {
                    println!("\n[info] File operations detected:");
                    for op in &analysis.file_ops {
                        println!(" - {}", op);
                    }
                }

                println!("\n[saferun] No changes were made.");
                return;
            }

            println!("[saferun] ⚠️  This is not a full sandbox.");
            println!("[saferun] Designed to reduce common risks when running unknown scripts.");

            println!("[saferun] Running script: {}", script);

            println!("[saferun] Isolation
    - filesystem: temporary
    - network: restricted (basic)
    - environment: clean");

            println!("\n[saferun] Risk Level: {}\n", analysis.risk_level);

            if !analysis.warnings.is_empty() {
                println!("[saferun] ⚠️  Potentially dangerous patterns detected:");
                for w in &analysis.warnings {
                    println!(" - {}", w.pattern);
                }
                
                println!("\nContinue? (y/N): ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let input = input.trim().to_lowercase();

                if input != "y" && input != "yes" {
                    println!("[saferun] Aborted.");
                    std::process::exit(0);
                }
            }

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