use clap::{App, Arg, ArgMatches};
use glob::glob;
use recyclebin_parser::RecycleBinParser;
use serde_json;
use std::convert::From;
use std::ffi::OsStr;
use std::path::Path;
use std::{
    fs::File,
    io::{self, Write},
};
enum OutputFormat {
    JSONL,
    CSV,
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s {
            "jsonl" => OutputFormat::JSONL,
            "csv" => OutputFormat::CSV,
            _ => OutputFormat::CSV,
        }
    }
}

fn output_data_csv(parsed_data: &RecycleBinParser) -> String {
    let sid = match parsed_data.sid.clone() {
        Some(sid) => sid,
        None => String::from(""),
    };
    return format!(
        "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",
        parsed_data.deletion_time,
        parsed_data.version,
        parsed_data.file_size,
        parsed_data.filename,
        sid,
    );
}

fn parse_cli_args() -> ArgMatches {
    App::new("RecycleBinParser-rs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("AbdulRhman Alfaifi - @A__ALFAIFI")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("PATH")
                .short('p')
                .long("path")
                .takes_value(true)
                .multiple(true)
                .help("Path(s) to Recycle Bin $I Files to be Parsed - accepts glob")
                .default_value("C:\\$Recycle.Bin\\*\\$I*"),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .default_value("stdout")
                .takes_value(true)
                .help("The file path to write the output to"),
        )
        .arg(
            Arg::with_name("output-format")
                .long("output-format")
                .takes_value(true)
                .possible_values(&["csv", "jsonl"])
                .default_value("jsonl")
                .help("Output format."),
        )
        .arg(
            Arg::with_name("extract-sid")
                .long("extract-sid")
                .takes_value(false)
                .help("Extract the SID by retriving the parent folder name"),
        )
        .arg(
            Arg::with_name("no-headers")
                .long("no-headers")
                .takes_value(false)
                .help("Don't print headers when using CSV as the output format"),
        )
        .get_matches()
}

fn main() {
    let args = parse_cli_args();
    let i_path = args.value_of("PATH").unwrap();
    let output_format = OutputFormat::from(args.value_of("output-format").unwrap());
    let output_to = args.value_of("output").unwrap();
    let mut output: Box<dyn Write> = match output_to {
        "stdout" => Box::new(io::stdout()),
        _ => Box::new(File::create(output_to).unwrap()),
    };

    let extract_sid = match args.occurrences_of("extract-sid") {
        0 => false,
        _ => true,
    };

    let mut headers_written = false;

    for entry in glob(i_path).expect("Failed to read glob pattren") {
        match entry {
            Ok(path) => {
                if args.occurrences_of("no-headers") == 0 && !headers_written {
                    if let OutputFormat::CSV = output_format {
                        match output
                            .write("DeletionTime,Version,FileSize,FileName,SID\n".as_bytes())
                        {
                            Ok(_) => {}
                            Err(e) => {
                                println!(
                                    "Unable to write CSV headers to '{output_to}', ERROR: {:?}",
                                    e
                                );
                            }
                        }
                        headers_written = true;
                    }
                }
                let mut f = File::open(&path).expect(&format!("Unable to open {:?}", &path));
                match RecycleBinParser::from_reader(&mut f) {
                    Ok(mut parser) => {
                        if extract_sid {
                            let parent_filename = path
                                .parent()
                                .unwrap_or(Path::new(""))
                                .file_name()
                                .unwrap_or(OsStr::new(""))
                                .to_string_lossy()
                                .to_string();
                            parser.set_sid(parent_filename);
                        }
                        match output_format {
                            OutputFormat::CSV => match output
                                .write(output_data_csv(&parser).as_bytes())
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Unable to write to '{output_to}', ERROR: {:?}", e);
                                }
                            },
                            OutputFormat::JSONL => {
                                match output.write(
                                    format!("{}\n", serde_json::to_string(&parser).unwrap())
                                        .as_bytes(),
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!(
                                            "Unable to write to '{output_to}', ERROR: {:?}",
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Unable to parse the file '{}', ERROR: {:?}",
                            &path.to_string_lossy(),
                            e
                        );
                    }
                }
            }
            Err(e) => println!("ERROR: {:?}", e),
        }
    }
    // let mut f = File::open("C:\\Users\\u0041\\Desktop\\$IK9O3HW.txt").unwrap();
}
