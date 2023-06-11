#[derive(Debug)]
pub enum DeleteIdentityError {
    Unknown,
    NotFound,
    BadRequest,
}