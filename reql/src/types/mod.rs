mod datetime;

use serde::Deserialize;
use serde_json::Value;

pub use datetime::DateTime;

#[derive(Debug, Deserialize)]
pub struct Change<OldVal = Value, NewVal = OldVal> {
    pub old_val: Option<OldVal>,
    pub new_val: Option<NewVal>,
    pub result_type: Option<String>,
    pub old_offset: Option<usize>,
    pub new_offset: Option<usize>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WriteStatus<OldVal = Value, NewVal = OldVal> {
    pub inserted: u32,
    pub replaced: u32,
    pub unchanged: u32,
    pub skipped: u32,
    pub deleted: u32,
    pub errors: u32,
    pub first_error: Option<String>,
    pub generated_keys: Option<Vec<uuid::Uuid>>,
    pub warnings: Option<Vec<String>>,
    pub changes: Option<Vec<Change<OldVal, NewVal>>>,
}
