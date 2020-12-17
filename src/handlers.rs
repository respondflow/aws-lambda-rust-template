use lambda_runtime::{error::HandlerError, Context};
use serde_derive::{Deserialize};
use simple_error::bail;
#[derive(Deserialize, Clone)]
pub struct EmptyEvent {}

pub fn default_handler(_e: EmptyEvent, _ctx: Context) -> Result<(), HandlerError> {
    bail!("FATAL: no Rust handler specified");
}

pub mod test {
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
}
