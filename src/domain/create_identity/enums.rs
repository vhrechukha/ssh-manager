#[derive(Debug)]
pub enum CreateIdentityError {
    BadRequest,
    Conflict,
    Unknown,
}