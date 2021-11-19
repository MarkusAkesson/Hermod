use hermod::cli;
use hermod::config::ClientConfigBuilder;
use hermod::consts::*;
use hermod::log::LogBuilder;
use hermod::request::RequestMethod;
use hermod::server::Server;

use std::net::SocketAddr;
use std::path::PathBuf;

use log::{error, info};

fn main() {
    let args = cli::get_matches();

    match args.subcommand() {
        ("server", Some(server_args)) => start_server(&server_args),
        ("upload", Some(req_args)) => exec_request(&req_args, RequestMethod::Upload),
        ("download", Some(req_args)) => exec_request(&req_args, RequestMethod::Download),
        ("gen-key", Some(gen_args)) => gen_key(&gen_args),
        ("share-key", Some(sk_args)) => share_key(&sk_args),
        _ => {}
    }
}

fn start_server(args: &clap::ArgMatches) {
    // Setup logging
    let daemonize = !args.is_present("no-daemon");
    let verbosity = args.occurrences_of("verbosity");

    let base_dir: PathBuf = [
        dirs::home_dir().expect("Failed to get home_directory"),
        HERMOD_BASE_DIR.into(),
    ]
    .iter()
    .collect();

    if !base_dir.exists() {
        std::fs::create_dir(base_dir).expect("Failed to create Hermods base directory");
    }

    match LogBuilder::with_verbosity(verbosity as u8)
        .set_stdout(!daemonize)
        .set_file(true)
        .init_logger()
    {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to initate logging, aborting. ({})", e);
            return;
        }
    }

    match args.subcommand() {
        ("setup", Some(args)) => {
            info!("Generating new static files for the server...");
            let force = args.is_present("force");
            Server::setup(force);
        }
        ("list", Some(_)) => {
            Server::list_known_clients();
        }
        // Treat all other cases as wanting to run the server
        _ => {
            let ip = args.value_of("ip").unwrap();
            let socket_addr = SocketAddr::new(ip.parse().unwrap(), hermod::consts::HERMOD_PORT);
            let workers = args.value_of("workers").unwrap();
            let workers = workers.parse::<u8>().unwrap();

            let server = Server::new(socket_addr, workers);

            if daemonize {
                match server.daemonize() {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Error: Failed to daemonize server: ({}).\n Aborting...", e);
                        return;
                    }
                }
            } else {
                std::env::set_current_dir(dirs::home_dir().expect("Failed to read home directory"))
                    .expect("Failed to set current working directory");
            }

            info!("Starting server");
            server.start();
        }
    }
}

fn exec_request(args: &clap::ArgMatches, method: RequestMethod) {
    let verbosity = args.occurrences_of("verbosity");

    match LogBuilder::with_verbosity(verbosity as u8)
        .set_stdout(true)
        .init_logger()
    {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to initate logging, aborting. ({})", e);
            return;
        }
    }

    let host = match hermod::host::load_host(args.value_of("remote").unwrap()) {
        Ok(host) => host,
        Err(err) => {
            log::error!("Unknown remote host: {}", err);
            return;
        }
    };
    let source: Vec<&str> = args
        .values_of("source")
        .expect("Obligatory argument 'source' missing, aborting")
        .collect();
    let destination = args
        .value_of("destination")
        .expect("Obligatory argument 'destination' missing, aborting");

    let cfg_builder = ClientConfigBuilder::new(&host)
        .source(&source)
        .destination(destination)
        .request(method);

    let cfg = cfg_builder.build_config();

    hermod::client::Client::new(cfg).execute();
}

fn gen_key(args: &clap::ArgMatches) {
    let verbosity = args.occurrences_of("verbosity");

    match LogBuilder::with_verbosity(verbosity as u8)
        .set_stdout(true)
        .init_logger()
    {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to initate logging, aborting. ({})", e);
            return;
        }
    }

    let alias = args.value_of("alias").expect("No alias provided, aborting");
    let force = args.is_present("force");
    let exists = hermod::host::exists(&alias);

    if exists && !force {
        log::error!("Found an existing host with that alias, to overwrite pass --force");
        return;
    } else if exists && force {
        log::warn!("Found an existing host with that alias, overwriting");
    }

    log::info!("Generating a new static keypair and a new identification token");

    let keys = hermod::genkey::gen_keys().expect("Failed to generate static keys");
    let private_key = keys.private;
    let public_key = keys.public;
    let id_token = hermod::genkey::gen_idtoken();

    let host = hermod::host::Host::with_alias(&alias)
        .set_id_token(&id_token)
        .set_public_key(&public_key)
        .set_private_key(&private_key);

    host.write_to_file()
        .expect("Failed to write generated key to file");
}

fn share_key(args: &clap::ArgMatches) {
    let verbosity = args.occurrences_of("verbosity");

    match LogBuilder::with_verbosity(verbosity as u8)
        .set_stdout(true)
        .init_logger()
    {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to initate logging, aborting. ({})", e);
            return;
        }
    }

    let name = args.value_of("name").expect("No name provided, aborting");
    let force = args.is_present("force");
    let exists = hermod::host::exists(&name);

    if exists && !force {
        log::error!("Found an existing host with that alias, to overwrite pass --force");
        return;
    } else if exists && force {
        log::warn!("Found an existing host with that alias, overwriting");
    }

    log::info!(
        "Generating and sharing a new key pair with the remote: {}",
        &name
    );

    let host = args
        .value_of("host")
        .expect("No host address provided, aborting");
    let host = hermod::host::Host::with_alias(&name).set_hostname(host);
    hermod::share_key::share_key(host);
}
