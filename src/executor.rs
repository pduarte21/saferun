use std::process::Command;
use std::io::{self, Write};
use tempfile::tempdir;

use crate::utils::contains_blocked_network_tools;

use crate::analyzer::AnalysisResult;

pub fn run_script(script: &str, contents: &str, analysis: &AnalysisResult) {
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

        print!("\nContinue? (y/N): ");
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
        eprintln!("[saferun] blocked: script uses forbidden network tool '{}'", tool);
        std::process::exit(1);
    }

    let dir = tempdir().expect("failed to create temp dir");
    let temp_path = dir.path();

    println!("[saferun] Using temp dir: {:?}", temp_path);

    let script_name = std::path::Path::new(script)
        .file_name()
        .expect("invalid script path");

    let temp_script_path = temp_path.join(script_name);

    std::fs::copy(script, &temp_script_path)
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