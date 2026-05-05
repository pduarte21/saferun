use std::{env, process};

pub enum CommandType {
    Run { script: String, dry_run: bool, json: bool },
}

fn print_usage() {
    eprintln!(
        "Usage:\n saferun run <script> [--dry-run] [--json]\n\nTry 'saferun run script.sh --dry-run'"
    );
}

pub fn parse_args() -> CommandType {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("run") => {
            let next = args.next();

            let script = match next {
                Some(s) if !s.starts_with("--") => s,
                _ => {
                    eprintln!("error: missing script argument\n");
                    print_usage();
                    process::exit(1);
                }
            };
            
            let remaining: Vec<String> = args.collect();
            let dry_run = remaining.contains(&"--dry-run".to_string());
            let json = remaining.contains(&"--json".to_string());

            CommandType::Run { script, dry_run, json }
        }
        Some(other) => {
            eprintln!("error: unknown subcommand '{}'\n", other);
            if other.contains(".sh") {
                eprintln!("Did you mean:
    saferun run {}?", other);
            } else {
                print_usage();
            }
            process::exit(1);
        }
        None => {
            eprintln!("error: missing subcommand\n");
            print_usage();
            process::exit(1);
        }
    }
}