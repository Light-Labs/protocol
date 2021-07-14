use std::{
    collections::HashMap,
    fs::File,
    iter::FromIterator,
    path::{Path, PathBuf},
};

use fehler::throws;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use stable_eyre::eyre::{Error, WrapErr};

trait FromCsv: Sized {
    #[throws]
    fn from_csv<P: AsRef<Path>>(path: P) -> Vec<Self>;
}

trait ProtocolEntry: DeserializeOwned + Sized {
    fn key(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    shorthand: String,
    name: String,
    dec: u8,
    description: String,
    implemented: Option<bool>,
    //
    // following props are specific to Response
    //
    initial_response: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorResponse {
    shorthand: String,
    name: String,
    dec: u8,
    description: String,
    implemented: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Command {
    shorthand: String,
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

impl ProtocolEntry for Response {
    fn key(&self) -> String {
        self.shorthand.to_owned()
    }
}
impl ProtocolEntry for ErrorResponse {
    fn key(&self) -> String {
        self.shorthand.to_owned()
    }
}
impl ProtocolEntry for Command {
    fn key(&self) -> String {
        self.shorthand.to_owned()
    }
}

struct ImplProto<T>(String, T)
where
    T: ProtocolEntry;

impl<V: ProtocolEntry> From<V> for ImplProto<V> {
    fn from(v: V) -> Self {
        ImplProto(v.key(), v)
    }
}

impl<V: ProtocolEntry> FromIterator<ImplProto<V>> for HashMap<String, V> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = ImplProto<V>>,
    {
        let mut commands: HashMap<String, V> = HashMap::new();
        for ImplProto(key, value) in iter {
            commands.insert(key, value);
        }
        commands
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RyderProtocol {
    commands: HashMap<String, Command>,
    commands_list: Vec<Command>,
    errors: HashMap<String, ErrorResponse>,
    errors_list: Vec<ErrorResponse>,
    responses: HashMap<String, Response>,
    responses_list: Vec<Response>,
}

impl RyderProtocol {
    #[throws]
    fn from_path(base_directory: PathBuf) -> Self {
        let base_directory = base_directory.join("csv");

        let commands_list = Command::from_csv(base_directory.join("commands.csv"))?;
        let commands = commands_list.iter().cloned().map(ImplProto::from).collect();

        let errors_list = FromCsv::from_csv(base_directory.join("error_responses.csv"))?;
        let errors = errors_list.iter().cloned().map(ImplProto::from).collect();

        let responses_list = FromCsv::from_csv(base_directory.join("responses.csv"))?;
        let responses = responses_list
            .iter()
            .cloned()
            .map(ImplProto::from)
            .collect();

        Self {
            commands,
            commands_list,
            errors,
            errors_list,
            responses,
            responses_list,
        }
    }
}

#[throws]
fn write_schema_json(protocol: &RyderProtocol, path: PathBuf) {
    let mut f =
        File::create(&path).wrap_err_with(|| format!("Unable to create file {:?}", &path))?;
    serde_json::to_writer_pretty(&mut f, &protocol)?;
    println!("Wrote {:?}", &path);
}

#[throws]
fn run(version_number: &str) {
    let base_directory: PathBuf = format!("../{}", &version_number).into();
    let protocol = RyderProtocol::from_path(base_directory.to_path_buf())?;
    let path = base_directory.join(format!("{}.json", &version_number));
    write_schema_json(&protocol, path)?;
}

#[throws]
fn main() {
    // navigate to `protocol/csv/` and run `cargo run -- 0.0.2`
    let version_number: String = std::env::args().nth(1).unwrap();
    run(&version_number)?;
}
