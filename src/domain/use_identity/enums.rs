#[derive(Debug)]
pub enum UseIdentityError {
    BadRequest,
    NotFound,
    Unknown,
}