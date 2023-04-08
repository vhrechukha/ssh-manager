mod cli;
mod domain;
mod repositories;

use repositories::config::InMemoryRepository;
use std::sync::Arc;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    cli::run(repo);
}