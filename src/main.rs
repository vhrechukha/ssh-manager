mod cli;
mod domain;
mod repositories;

use std::sync::Arc;
use repositories::config::{Repository, FileRepository};

fn main() {
    let repository: Arc<dyn Repository> = Arc::new(FileRepository::new());

    cli::run(repository);
}