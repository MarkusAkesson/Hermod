use hermod::cli;
use hermod::config::ClientConfigBuilder;
use hermod::consts::*;
use hermod::request::RequestMethod;
use hermod::server::HermodServer;

use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;

use daemonize::Daemonize;
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

    match hermod::log::setup_logger(!daemonize, verbosity) {
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
            HermodServer::setup(force);
        }
        ("list", Some(_)) => {
            HermodServer::list_known_clients();
        }
        // Treat all other cases as wanting to run the server
        _ => {
            // Move this to HermodServer?
            if daemonize {
                info!("Preparing to run server as a daemon");
                let stdout = File::create("/tmp/hermod.out").unwrap();
                let stderr = File::create("/tmp/hermod.err").unwrap();
                let daemon = Daemonize::new()
                    .pid_file("/tmp/hermod.pid")
                    .working_directory(dirs::home_dir().expect("Could not find the home directory"))
                    .stdout(stdout)
                    .stderr(stderr);

                match daemon.start() {
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

            let ip = args.value_of("ip").unwrap();
            let socket_addr = SocketAddr::new(ip.parse().unwrap(), hermod::consts::HERMOD_PORT);
            info!("Starting server");
            HermodServer::run_server(socket_addr);
        }
    }
}

fn exec_request(args: &clap::ArgMatches, method: RequestMethod) {
    let host = match hermod::host::load_host(args.value_of("remote").unwrap()) {
        Ok(host) => host,
        Err(err) => {
            eprintln!("Unknown remote host: {}", err);
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

    hermod::client::HermodClient::new(cfg).execute();
}

fn gen_key(args: &clap::ArgMatches) {
    let alias = args.value_of("alias").expect("No alias provided, aborting");
    let force = args.is_present("force");
    let exists = hermod::host::exists(&alias);

    if exists && !force {
        eprintln!("Found an existing host with that alias, to overwrite pass --force");
        return;
    } else if exists && force {
        println!("Found an existing host with that alias, overwriting");
    }

    println!("Generating a new static keypair and a new identification token");

    let keys = hermod::genkey::gen_keys().expect("Failed to generate static keys");
    let private_key = keys.private;
    let public_key = keys.public;
    let id_token = hermod::genkey::gen_idtoken();

    let host = hermod::host::Host::with_alias(&alias)
        .set_id_token(&id_token)
        .set_public_key(&public_key)
        .set_private_key(&private_key);

    println!("{}", host);

    host.write_to_file()
        .expect("Failed to write generated key to file");
}

fn share_key(args: &clap::ArgMatches) {
    let name = args.value_of("name").expect("No name provided, aborting");
    let force = args.is_present("force");
    let exists = hermod::host::exists(&name);

    if exists && !force {
        eprintln!("Found an existing host with that alias, to overwrite pass --force");
        return;
    } else if exists && force {
        println!("Found an existing host with that alias, overwriting");
    }

    println!(
        "Generating and sharing a new key pair with the remote: {}",
        &name
    );

    let host = args
        .value_of("host")
        .expect("No host address provided, aborting");
    let host = hermod::host::Host::with_alias(&name).set_hostname(host);
    hermod::share_key::share_key(host);
}
