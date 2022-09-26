use clap::{crate_version, App, Arg};
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Data {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct File {
    pub name: String,
    pub size: u64, 
}

fn main() {
    let app = App::new("sysmon-rs")
        .version(crate_version!())
        .author("Chris Marquis")
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .help("The base path to discover files")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Pretty prints the output")
        );
    let matches = app.clone().get_matches();

    let mut data = Data{..Default::default()};
    for entry in WalkDir::new(matches.value_of("path").unwrap_or_default())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        data.files.push(File{name: entry.path().display().to_string(), size: entry.metadata().unwrap().len()});
    };

    println!("{}", serde_json::to_string(&data).unwrap());
}
