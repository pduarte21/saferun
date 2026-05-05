use std::path::Path;
use std::process::{Command, exit};
use std::io::{self, Write};
use tempfile::tempdir;

use crate::utils::contains_blocked_network_tools;

use crate::analyzer::AnalysisResult;

pub fn run_script(script: &str, contents: &str, analysis: &AnalysisResult) {
    println!("[saferun] Running script: {}", script);
    println!("----------------------------------------");

    let risk_display = match analysis.risk_level.as_str() {
        "HIGH" => "HIGH 🚨",
        "MEDIUM" => "MEDIUM ⚠️",
        "LOW" => "LOW ✓",
        _ => &analysis.risk_level,
    };

    println!("\n[saferun] Risk: {}", risk_display);

    if !analysis.warnings.is_empty() {
        println!("[saferun] Potential risks detected:");
        for w in &analysis.warnings {
            println!(" - {}", w.pattern);
        }
        println!("\n----------------------------------------");
        print!("Continue? (y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("[saferun] Aborted.");
            std::process::exit(0);
        }
    }

    if let Some(tool) = contains_blocked_network_tools(contents) {
        println!("[warning] Script uses network tool: {}", tool);
    }

    let dir = tempdir().expect("failed to create temp dir");
    let temp_path = dir.path();

    println!("[saferun] Using temp dir: {:?}", temp_path);

    let script_name = std::path::Path::new(script)
        .file_name()
        .expect("invalid script path");

    let temp_script_path = temp_path.join(script_name);

    if let Err(e) = std::fs::copy(script, &temp_script_path) {
        eprintln!("error: failed to copy script: {}", e);
        exit(1);
    }

    let ext = Path::new(script)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let mut command = match ext {
        "sh" => {
            let mut cmd = Command::new("sh");
            cmd.arg(&script_name);
            cmd
        }
        "ps1" => {
            let mut cmd = Command::new("pwsh");
            cmd.arg("-File").arg(&script_name);
            cmd
        }
        "bat" => {
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", script_name.to_str().unwrap()]);
            cmd
        }
        _ => {
            eprintln!("error: unsupported script type '.{}'", ext);
            exit(1);
        }
    };

    command
        .current_dir(temp_path)
        .env_clear()
        .env("HOME", temp_path);

    if cfg!(target_os = "windows") {
        command.env("PATH", "C:\\Windows\\System32;C:\\Windows");
    } else {
        command.env("PATH", "/usr/bin:/bin");
    }

    let output = match command.output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: failed to execute script: {}", e);
            exit(1);
        }
    };

    println!("[saferun] Output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}