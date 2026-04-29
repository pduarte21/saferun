#[derive(Clone)]
pub struct Warning {
    pub pattern: String,
}

pub struct AnalysisResult {
    pub command_count: usize,
    pub warnings: Vec<Warning>,
    pub network_usage: bool,
    pub file_ops: Vec<String>,
    pub risk_level: String,
}

pub fn analyze_script(contents: &str) -> AnalysisResult {
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


pub fn severity(pattern: &str) -> &'static str {
    match pattern {
        "rm -rf" | "dd " | "mkfs"       => "HIGH",
        "chmod 777"                     => "MEDIUM",
        "curl" | "wget" | "nc " | "ssh" => "INFO",
        "piped execution"               => "HIGH",
        _                               => "INFO", 
    }
}

pub fn explain_pattern(pattern: &str) -> &'static str {
    match pattern {
        "rm -rf"            => "deletes files recursively (can wipe directories)",
        "chmod 777"         => "makes files globally writable",
        "dd "               => "low-level disk write (can corrupt data)",
        "mkfs"              => "formats a filesystem (destructive)",
        "curl" | "wget"     => "downloads remote content",
        "nc " | "netcat"    => "opens raw network connections",
        "ssh"               => "connects to remote machines",
        "piped execution"   => "executes piped or downloaded content",
        _                   => "potentially unsafe operation",
    }
}


pub fn severity_weight(pattern: &str) -> u8 {
    match severity(pattern) {
        "HIGH"   => 3,
        "MEDIUM" => 2,
        "INFO"   => 1,
        _        => 0,
    }
}

fn print_group(name: &str, warnings: &Vec<&Warning>) {
    if warnings.is_empty() {
        return;
    }

    println!("\n[{}]", name);

    for w in warnings {
        let pattern = &w.pattern;

        let explanation = if pattern.starts_with("sensitive path") {
            "accesses potentially sensitive system data"
        }
        else {
            explain_pattern(pattern)
        };

        println!(" - {} -> {}", pattern, explanation);
    }
}

pub fn print_dry_run(analysis: &AnalysisResult) {
    println!("[saferun] ⚠️  This is a best-effort preview. Bash scripts can be dynamic.\n");
    println!("[saferun] Dry run mode (no execution)\n");
    println!("[saferun] Risk Level: {}\n", analysis.risk_level);

    println!("[info] Script summary:");
    println!(" - commands: {}", analysis.command_count);

    if !analysis.warnings.is_empty() {
        println!("\n[warning] Potentially dangerous patterns:");

        let mut high = Vec::new();
        let mut medium = Vec::new();
        let mut info = Vec::new();

        for w in &analysis.warnings {
            let pattern = &w.pattern;

            let sev = if pattern.starts_with("sensitive path") {
                "HIGH"
            } else {
                severity(pattern)
            };

            match sev {
                "HIGH"   => high.push(w),
                "MEDIUM" => medium.push(w),
                _        => info.push(w),
            }
        }
        
        print_group("HIGH", &high);
        print_group("MEDIUM", &medium);
        print_group("INFO", &info);
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
}