use clap::{App, Arg};

#[derive(Debug)]
pub struct Args {
    pub destination_name: String,
    pub destination_port: String,
    pub window_size: String,
    pub infile: String,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("acp_sender")
            .arg(
                Arg::with_name("destination_name")
                    .short("d")
                    .long("destination_name")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("destination_port")
                    .short("p")
                    .long("destination_port")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("window_size")
                    .short("w")
                    .long("window_size")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("infile")
                    .short("f")
                    .long("infile")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();

        let destination_name = matches.value_of("destination_name").unwrap().to_string();

        let destination_port = matches.value_of("destination_port").unwrap().to_string();

        let window_size = matches.value_of("window_size").unwrap().to_string();

        let infile = matches.value_of("infile").unwrap().to_string();

        Self {
            destination_name,
            destination_port,
            window_size,
            infile,
        }
    }
}
