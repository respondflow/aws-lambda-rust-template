use lambda_runtime::{error::HandlerError, Context};
use serde_derive::{Deserialize};
use simple_error::bail;
#[derive(Deserialize, Clone)]
pub struct EmptyEvent {}

pub mod test;

pub fn default_handler(_e: EmptyEvent, _ctx: Context) -> Result<(), HandlerError> {
    bail!("FATAL: no Rust handler specified");
}
