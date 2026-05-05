mod cli;
mod analyzer;
mod executor;
mod utils;

use cli::parse_args;
use analyzer::{analyze_script, print_dry_run};
use executor::run_script;

use crate::analyzer::to_json_output;

fn main() {
    let cmd = parse_args();

    match cmd {
        cli::CommandType::Run { script, dry_run, json } => {
            let contents = match std::fs::read_to_string(&script) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: failed to read script '{}'\n{}", script, e);
                    std::process::exit(1);
                }
            };
                

            let analysis = analyze_script(&contents);

            if dry_run {
                if json {
                    let output = to_json_output(&analysis);
                    println!("{}", serde_json::to_string_pretty(&output).unwrap());
                    return;
                }
                print_dry_run(&analysis);
                return;
            }

            run_script(&script, &contents, &analysis);
        }
    }
}