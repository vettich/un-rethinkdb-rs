mod datetime;

use serde::Deserialize;
use serde_json::Value;

pub use datetime::DateTime;

#[derive(Debug, Deserialize)]
pub struct Change<O = Value, N = Value> {
    pub old_val: Option<O>,
    pub new_val: Option<N>,
    pub result_type: Option<String>,
    pub old_offset: Option<usize>,
    pub new_offset: Option<usize>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WriteStatus<T = Value, U = Value> {
    pub inserted: u32,
    pub replaced: u32,
    pub unchanged: u32,
    pub skipped: u32,
    pub deleted: u32,
    pub errors: u32,
    pub first_error: Option<String>,
    pub generated_keys: Option<Vec<uuid::Uuid>>,
    pub warnings: Option<Vec<String>>,
    pub changes: Option<Vec<Change<T, U>>>,
}
