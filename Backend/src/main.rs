extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::env::VarError;
use core::fmt;
use std::fmt::Formatter;

struct AppError(String);

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

type AppResult = std::result::Result<(), AppError>;

impl From<VarError> for AppError {
    fn from(_: VarError) -> Self {
        AppError("Not all required parameters provided. See README file.".to_string())
    }
}

fn main() -> AppResult {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;

    AppResult::Ok(())
}
