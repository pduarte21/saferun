mod cli;
mod analyzer;
mod executor;
mod utils;

use cli::parse_args;
use analyzer::{analyze_script, print_dry_run};
use executor::run_script;

fn main() {
    let cmd = parse_args();

    match cmd {
        cli::CommandType::Run { script, dry_run } => {
            let contents = std::fs::read_to_string(&script)
                .expect("failed to read script");

            let analysis = analyze_script(&contents);

            if dry_run {
                print_dry_run(&analysis);
                return;
            }

            run_script(&script, &contents, &analysis);
        }
    }
}