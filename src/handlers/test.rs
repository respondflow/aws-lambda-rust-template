use lambda_runtime::{Context, error::HandlerError};
use serde_derive::{Serialize};

use super::EmptyEvent;

#[derive(Serialize, Clone)]
pub struct CustomOutput {
    message: String,
}

pub fn handler(_e: EmptyEvent, _ctx: Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        message: String::from("Hello, world!")
    })
}
