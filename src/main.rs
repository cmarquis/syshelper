use std::path::PathBuf;

use clap::{crate_version, App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

//todo could handle all errors and output error messages vs using unwraps and expects

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
        .subcommand(
            SubCommand::with_name("ls")
                .about("Returns the files and their sizes from the specified directory")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .short("p")
                        .help("The base path to discover files")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Pretty prints the output"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .help("The file to output the results to")
                .takes_value(true),
        );
    let matches = app.clone().get_matches();

    let mut data = Data {
        ..Default::default()
    };

    if let Some(subc) = matches.subcommand_matches("ls") {
        for entry in WalkDir::new(subc.value_of("path").unwrap_or_default())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            data.files.push(File {
                name: entry.path().display().to_string(),
                size: entry.metadata().unwrap().len(),
            });
        }
    }
    if matches.is_present("output") {
        let json = if matches.is_present("pretty") {
            serde_json::to_string_pretty(&data).unwrap()
        } else {
            serde_json::to_string(&data).unwrap()
        };
        let path = PathBuf::from(matches.value_of("output").expect("invalid path"));
        //todo could check if file path exists and create it if not
        std::fs::write(path, json).expect("unable to write output to file");
    } else if matches.is_present("pretty") {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    } else {
        println!("{}", serde_json::to_string(&data).unwrap());
    }
}
