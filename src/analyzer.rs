use serde::Serialize;

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

#[derive(Serialize)]
pub struct JsonWarning {
    pattern: String,
    severity: String,
    explanation: String,
}

#[derive(Serialize)]
pub struct JsonFileOp {
    path: String,
    operation: String,
}

#[derive(Serialize)]
pub struct JsonOutput {
    risk_level: String,
    command_count: usize,
    warnings: Vec<JsonWarning>,
    network_usage: bool,
    file_operations: Vec<JsonFileOp>,
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
        "rm -rf" | "dd" | "mkfs"       => "HIGH",
        "chmod 777"                     => "MEDIUM",
        "curl" | "wget" | "nc" | "ssh" => "INFO",
        "piped execution"               => "HIGH",
        _                               => "INFO", 
    }
}

pub fn explain_pattern(pattern: &str) -> &'static str {
    match pattern {
        "rm -rf"            => "deletes files recursively (can wipe directories)",
        "chmod 777"         => "makes files globally writable",
        "dd"               => "low-level disk write (can corrupt data)",
        "mkfs"              => "formats a filesystem (destructive)",
        "curl" | "wget"     => "downloads remote content",
        "nc" | "netcat"    => "opens raw network connections",
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
    println!("[saferun] Preview mode (best-effort analysis)");
    println!("[saferun] Dry run mode (no execution)");
    println!("----------------------------------------");
    let risk_display = match analysis.risk_level.as_str() {
        "HIGH" => "HIGH 🚨",
        "MEDIUM" => "MEDIUM ⚠️",
        "LOW" => "LOW ✓",
        _ => &analysis.risk_level,
    };

    println!("[saferun] Risk: {}", risk_display);

    if !analysis.warnings.is_empty() {
        println!("\n[warning] Potential risks detected:");

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

    if !analysis.file_ops.is_empty() {
        println!("\n[info] File operations:");

        for line in &analysis.file_ops {
            if let Some(op) = parse_file_op(line) {
                if op.operation == "delete" {
                    println!(" - {} ({}) ⚠️", op.path, op.operation);
                } else {
                    println!(" - {} ({})", op.path, op.operation);
                }
            } else {
                println!(" - {}", line);
            }
        }
    }

    if analysis.network_usage {
        println!("\n[info] Network access: detected");
    }

    println!("\n[info] Script summary:");
    println!(" - commands: {}", analysis.command_count);

    println!("\n----------------------------------------");
    println!("[saferun] No changes were made.");
}

fn parse_file_op(line: &str) -> Option<JsonFileOp> {
    // write (>)
    if let Some(pos) = line.find('>') {
        let path = line[pos + 1..].trim();
        return Some(JsonFileOp {
            path: path.to_string(),
            operation: "write".to_string(),
        });
    }

    // append (>>)
    if let Some(pos) = line.find(">>") {
        let path = line[pos + 2..].trim();
        return Some(JsonFileOp {
            path: path.to_string(),
            operation: "append".to_string(),
        });
    }

    // rm
    if line.starts_with("rm ") {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // último argumento normalmente é o path
        if let Some(path) = parts.last() {
            return Some(JsonFileOp {
                path: path.to_string(),
                operation: "delete".to_string(),
            });
        }
    }

    // cp
    if line.starts_with("cp ") {
        return Some(JsonFileOp {
            path: line.replace("cp ", "").trim().to_string(),
            operation: "copy".to_string(),
        });
    }

    // mv
    if line.starts_with("mv ") {
        return Some(JsonFileOp {
            path: line.replace("mv ", "").trim().to_string(),
            operation: "move".to_string(),
        });
    }

    None
}

pub fn to_json_output(analysis: &AnalysisResult) -> JsonOutput {
    let warnings = analysis.warnings.iter().map(|w| {
        let pattern = &w.pattern;

        let severity = if pattern.starts_with("sensitive path") {
            "HIGH".to_string()
        } else {
            severity(pattern).to_string()
        };

        let explanation = if pattern.starts_with("sensitive path") {
            "accesses potentially sensitive system data".to_string()
        } else {
            explain_pattern(pattern).to_string()
        };

        JsonWarning {
            pattern: pattern.clone(),
            severity,
            explanation
        }
    }).collect();

    let file_operations = analysis.file_ops.iter()
        .filter_map(|line| parse_file_op(line))
        .collect();

    JsonOutput { 
        risk_level: analysis.risk_level.clone(),
        command_count: analysis.command_count,
        warnings,
        network_usage: analysis.network_usage,
        file_operations 
    }
}