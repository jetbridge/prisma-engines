extern crate log;
extern crate slog;
#[macro_use]
extern crate slog_scope;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate debug_stub_derive;

mod cli;
mod context;
mod data_model_loader;
mod dmmf; // Temporary
mod error;
mod exec_loader;
mod request_handlers;
mod serializers;
mod server;
mod utilities;

use clap::{App as ClapApp, Arg, SubCommand};
use cli::*;
use error::*;
use request_handlers::{PrismaRequest, RequestHandler};
use server::HttpServer;
use std::{env, error::Error, process};
use tokio::runtime::Builder;

pub type PrismaResult<T> = Result<T, PrismaError>;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let matches = ClapApp::new("Prisma Query Engine")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("port")
                .help("The port the query engine should bind to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("legacy")
                .long("legacy")
                .help("Switches query schema generation to Prisma 1 compatible mode.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Prints the server commit ID")
                .takes_value(false)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("cli")
                .about("Doesn't start a server, but allows running specific commands against Prisma.")
                .arg(
                    Arg::with_name("dmmf")
                        .long("dmmf")
                        .help("Output the DMMF from the loaded data model.")
                        .takes_value(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("dmmf_to_dml")
                        .long("dmmf_to_dml")
                        .help("Convert the DMMF to a data model")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("get_config")
                        .long("get_config")
                        .help("Get the configuration from the given data model")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .get_matches();

    if matches.is_present("version") {
        println!(env!("GIT_HASH"));
    } else if let Some(matches) = matches.subcommand_matches("cli") {
        match CliCommand::new(matches) {
            Some(cmd) => {
                if let Err(err) = cmd.execute() {
                    info!("Encountered error during initialization:");
                    err.pretty_print();
                    process::exit(1);
                }
            }
            None => {
                error!("No command provided");
                process::exit(1);
            }
        }
    } else {
        // let _logger = Logger::build("prisma"); // out from the scope until finding faster logger
        pretty_env_logger::init();

        let port = matches
            .value_of("port")
            .map(|p| p.to_owned())
            .or_else(|| env::var("PORT").ok())
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or_else(|| 4466);

        let address = ([0, 0, 0, 0], port);
        let legacy = matches.is_present("legacy");

        let rt = Builder::new().blocking_threads(num_cpus::get_physical() * 2 + 1).build()?;
        let result = rt.block_on(async { HttpServer::run(address, legacy).await });

        if let Err(err) = result {
            info!("Encountered error during initialization:");
            err.pretty_print();
            process::exit(1);
        };
    };

    Ok(())
}
