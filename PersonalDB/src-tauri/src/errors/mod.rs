use thiserror::Error;
use sea_orm::DbErr;
use serde_json::Error;

// Handles database errors
#[derive(Debug, thiserror::Error)]
pub enum ItemDBError {
#[error(transparent)]
Io(#[from] std::io::Error),
#[error("Error accessing or modifying database")]
DbErr(#[from] sea_orm::DbErr),
#[error("Error trying from integer")]
TryFromIntError(#[from] std::num::TryFromIntError),
#[error("Error managing serde_json")]
Error(#[from] serde_json::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for ItemDBError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: serde::ser::Serializer,
    {
    serializer.serialize_str(self.to_string().as_ref())
    }
}