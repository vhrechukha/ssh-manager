mod cli;
mod domain;
mod repositories;

use clap::{App, Arg};
use repositories::config::InMemoryRepository;
use std::sync::Arc;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new("Ssh-Manager")
        .arg(Arg::with_name("cli").long("cli").help("Runs in CLI mode"))
        .get_matches();

    match matches.occurrences_of("cli") {
        _ => cli::run(repo),
    }
}