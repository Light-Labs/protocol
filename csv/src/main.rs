use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

trait FromCsv<T> {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<T>>;
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    prefix: String,
    name: String,
    dec: u8,
    description: String,
    implemented: Option<bool>,
    //
    // following props are specific to Response
    //
    initial_response: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    prefix: String,
    name: String,
    dec: u8,
    description: String,
    implemented: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    prefix: String,
    name: String,
    dec: u8,
    description: String,
    implemented: Option<bool>,
    //
    // following props are specific to Command
    //
    user_confirm: Option<bool>,
    send_input: Option<bool>,
    enabled: Option<bool>,
    temporary: bool,
}

impl FromCsv<Response> for Response {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        let mut results = vec![];
        for result in rdr.deserialize() {
            let record: Self = result?;
            results.push(record);
        }
        Ok(results)
    }
}

impl FromCsv<ErrorResponse> for ErrorResponse {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        let mut results = vec![];
        for result in rdr.deserialize() {
            let record: Self = result?;
            results.push(record);
        }
        Ok(results)
    }
}

impl FromCsv<Command> for Command {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        let mut results = vec![];
        for result in rdr.deserialize() {
            let record: Self = result?;
            results.push(record);
        }
        Ok(results)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RyderProtocol {
    commands: Vec<Command>,
    errors: Vec<ErrorResponse>,
    responses: Vec<Response>,
}

impl RyderProtocol {
    fn from_path(base_directory: PathBuf) -> Result<Self> {
        let commands = Command::from_csv(base_directory.join("commands.csv"))?;
        let errors = ErrorResponse::from_csv(base_directory.join("error_responses.csv"))?;
        let responses = Response::from_csv(base_directory.join("responses.csv"))?;
        Ok(Self {
            commands,
            errors,
            responses,
        })
    }
}

fn main() {
    // navigate to `protocol/csv/` and run `cargo run -- ../0.0.2/csv`
    let base_directory: PathBuf = std::env::args_os().nth(1).unwrap().into();
    let protocol = RyderProtocol::from_path(base_directory).unwrap();
    println!("{:#?}", protocol);
}
