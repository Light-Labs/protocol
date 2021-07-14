use std::{
    fs::File,
    path::{Path, PathBuf},
};

use fehler::throws;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use stable_eyre::eyre::{Error, WrapErr};

trait FromCsv: Sized {
    #[throws]
    fn from_csv<P: AsRef<Path>>(path: P) -> Vec<Self>;
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

impl<T> FromCsv for T
where
    T: DeserializeOwned,
{
    #[throws]
    fn from_csv<P: AsRef<Path>>(path: P) -> Vec<Self> {
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        let mut results = vec![];
        for result in rdr.deserialize() {
            let record: Self = result?;
            results.push(record);
        }
        results
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RyderProtocol {
    commands: Vec<Command>,
    errors: Vec<ErrorResponse>,
    responses: Vec<Response>,
}

impl RyderProtocol {
    #[throws]
    fn from_path(base_directory: PathBuf) -> Self {
        let commands = FromCsv::from_csv(base_directory.join("commands.csv"))?;
        let errors = FromCsv::from_csv(base_directory.join("error_responses.csv"))?;
        let responses = FromCsv::from_csv(base_directory.join("responses.csv"))?;

        Self {
            commands,
            errors,
            responses,
        }
    }
}

#[throws]
fn run(base_directory: PathBuf) {
    let protocol = RyderProtocol::from_path(base_directory.to_path_buf())?;
    let path = base_directory.join("v.json");
    let mut f =
        File::create(&path).wrap_err_with(|| format!("Unable to create file {:?}", &path))?;
    serde_json::to_writer_pretty(&mut f, &protocol)?;
    println!("Wrote {:?}", &path);
}

#[throws]
fn main() {
    // navigate to `protocol/csv/` and run `cargo run -- ../0.0.2/csv`
    run(std::env::args_os().nth(1).unwrap().into())?;
}
