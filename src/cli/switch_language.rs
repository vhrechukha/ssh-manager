use crate::repositories::traits::Repository;

use crate::domain::switch_language::switch_language::execute;

use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) -> Result<(), Box<dyn std::error::Error>> {
    execute(repo.clone())?;
    
    Ok(())
}

