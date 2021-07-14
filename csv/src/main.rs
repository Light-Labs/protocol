use serde::Deserialize;

trait FromCsv<T> {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    prefix: String,
    name: String,
    dec: u8,
    description: String,
    implemented: Option<bool>,
}

#[derive(Debug, Deserialize)]
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
    fn from_csv<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
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
    fn from_csv<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
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
    fn from_csv<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        let mut results = vec![];
        for result in rdr.deserialize() {
            let record: Self = result?;
            results.push(record);
        }
        Ok(results)
    }
}

fn main() {
    // navigate to `protocol/csv/` and run `cargo run -- ../0.0.2/csv`
    let base_directory: std::path::PathBuf = std::env::args_os().nth(1).unwrap().into();
    let commands = Command::from_csv(base_directory.join("commands.csv")).unwrap();
    let errors = ErrorResponse::from_csv(base_directory.join("error_responses.csv")).unwrap();
    let responses = Response::from_csv(base_directory.join("responses.csv")).unwrap();
    println!("{:#?}", commands);
    println!("{:#?}", responses);
    println!("{:#?}", errors);
}
