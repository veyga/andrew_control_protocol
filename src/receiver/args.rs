use clap::{App, Arg};

#[derive(Debug)]
pub struct Args {
    pub listening_port: String,
    pub window_size: String,
    pub outfile: String,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("acp_receiver")
            .arg(
                Arg::with_name("listening_port")
                    .short("p")
                    .long("listening_port")
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
                Arg::with_name("outfile")
                    .short("f")
                    .long("outfile")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();

        let listening_port = matches.value_of("listening_port").unwrap().to_string();

        let window_size = matches.value_of("window_size").unwrap().to_string();

        let outfile = matches.value_of("outfile").unwrap().to_string();

        Self {
            listening_port,
            window_size,
            outfile,
        }
    }
}
