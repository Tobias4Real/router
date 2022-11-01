use std::env;
use std::fmt::{Display, Formatter};
use std::process::exit;

#[derive(Debug)]
pub enum ArgParseError {
    MissingArgumentFor(&'static str),
    UnknownArgument(String),
    IllegalArgument(&'static str),
}


pub mod flag {
    pub type Type = u32;

    pub const SHOW_NAIVE_NODE: Type = 1;
}

const FLAGS: [(&str, flag::Type); 1] = [("--naive", flag::SHOW_NAIVE_NODE)]; 

impl Display for ArgParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let help = "\nUse --help for more infos and examples.";
        match self {
            ArgParseError::MissingArgumentFor(arg) => {
                write!(f, "Error parsing arguments | Missing argument for {}{}", arg, help)
            }
            ArgParseError::UnknownArgument(arg) => {
                write!(f, "Error parsing arguments | Unknown argument '{}'{}", arg, help)
            }
            ArgParseError::IllegalArgument(err) => {
                write!(f, "Error parsing arguments | {}{}", err, help)
            }
        }
    }
}

pub struct Args {
    pub cmd: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub graph_file: Option<String>,
    pub query_file: Option<String>,
    pub source_node: Option<i64>,
    pub target_node: Option<i64>,
    pub flags: flag::Type,
}

impl Args {
    fn empty() -> Self {
        Self {
            cmd: "".to_string(),
            lat: None,
            lon: None,
            graph_file: None,
            query_file: None,
            source_node: None,
            target_node: None,
            flags: 0,
        }
    }

    pub fn parse() -> Result<Self, ArgParseError> {
        let mut iter = env::args();
        let mut result = Self::empty();
        //First argument should be never empty, so unwrap is ok
        result.cmd = iter.next().unwrap();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--help" => {
                    println!(include_str!("help.txt"));

                    exit(0);
                }
                "-lat" => {
                    result.lat = Some(
                        iter.next()
                            .ok_or(ArgParseError::MissingArgumentFor("-lat"))?
                            .parse::<f64>().map_err(|_| ArgParseError::IllegalArgument("-lat: Wrong format. Expected Something like '48.7758'."))?
                    );
                }
                "-lon" => {
                    result.lon = Some(
                        iter.next()
                            .ok_or(ArgParseError::MissingArgumentFor("-lon"))?
                            .parse::<f64>().map_err(|_| ArgParseError::IllegalArgument("-lon: Wrong format. Expected Something like '9.1829'."))?
                    );
                }
                "-graph" => {
                    result.graph_file = Some(
                        iter.next().ok_or(ArgParseError::MissingArgumentFor("-graph"))?.clone()
                    );
                }
                "-que" => {
                    result.query_file = Some(
                        iter.next().ok_or(ArgParseError::MissingArgumentFor("-que"))?.clone()
                    );
                }
                "-s" => {
                    result.source_node = Some(
                        iter.next()
                            .ok_or(ArgParseError::MissingArgumentFor("-s"))?
                            .parse::<i64>().map_err(|_| ArgParseError::IllegalArgument("-s: Wrong format. Expected something like '638394'."))?
                    );
                },
                "-t" => {
                    result.target_node = Some(
                        iter.next()
                            .ok_or(ArgParseError::MissingArgumentFor("-t"))?
                            .parse::<i64>().map_err(|_| ArgParseError::IllegalArgument("-s: Wrong format. Expected something like '8371825'."))?
                    );
                }
                _ => {
                    if !FLAGS
                    .iter()
                    .filter(|(str, _)| arg == *str)
                    .any(|(_, flag)| {
                        result.flags |= *flag;
                        true
                    }) {
                        return Err(ArgParseError::UnknownArgument(arg.clone()));            
                    }

                }
            }
        }
        Ok(result)
    }
}