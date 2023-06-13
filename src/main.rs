mod cli;
mod domain;
mod repositories;
mod infrastructure;

use std::sync::Arc;
use repositories::config::{FileRepository};
use crate::repositories::traits::Repository;


fn main() {
    let repository: Arc<dyn Repository> = Arc::new(FileRepository::new());

    cli::run(repository);
}