//use std::path::PathBuf;
use thiserror::Error;
use holochain_serialized_bytes::SerializedBytesError;
use holochain::core::ribosome::error::RibosomeError;
use holochain::conductor::error::*;

pub type SnapmailApiResult<T> = Result<T, SnapmailApiError>;

#[derive(Error, Debug)]
pub enum SnapmailApiError {
   #[error("data store disconnected")]
   Disconnect(#[from] std::io::Error),
   #[error("Internal serialization error: {0}")]
   SerializedBytesError(#[from] SerializedBytesError),
   #[error(transparent)]
   RibosomeError(#[from] RibosomeError),
   #[error(transparent)]
   ConductorError(#[from] ConductorError),
   #[error("Holochain call timed out")]
   Timeout,
   #[error("Unauthorized zome call")]
   Unauthorized,
   #[error("Network error: {0}")]
   NetworkError(String),
   #[error("unknown data store error")]
   Unknown,
}