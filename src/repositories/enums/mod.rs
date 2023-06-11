pub mod add_identity_error;
pub mod delete_identity_error;
pub mod find_identities_error;
pub mod find_identity_error;

pub use add_identity_error::AddIdentityRepositoryError as AddIdentityRepositoryError;
pub use delete_identity_error::DeleteIdentityRepositoryError as DeleteIdentityRepositoryError;
pub use find_identities_error::FindIdentitiesRepositoryError as FindIdentitiesRepositoryError;
pub use find_identity_error::FindIdentityRepositoryError as FindIdentityRepositoryError;
