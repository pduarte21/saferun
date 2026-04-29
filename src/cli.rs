use std::env;

pub enum CommandType {
    Run { script: String, dry_run: bool },
}

pub fn parse_args() -> CommandType {
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