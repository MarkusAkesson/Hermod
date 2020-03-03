use hermod::cli;
use hermod::config::ClientConfigBuilder;
use hermod::request::RequestMethod;
use hermod::server::HermodServer;

use std::fs::File;
use std::net::SocketAddr;

use daemonize::Daemonize;
use log::info;

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
    match args.subcommand() {
        ("setup", Some(args)) => {
            println!("Genereting new static files for the server...");
            let force = args.is_present("force");
            HermodServer::setup(force);
        }
        ("list", Some(_)) => {
            HermodServer::list_known_clients();
        }
        // Treat all other cases as wanting to run the server
        _ => {
            // Move this to HermodServer?
            let verbosity = args.occurrences_of("verbosity");
            if !args.is_present("no-daemon") {
                println!("Preparing to run server as a daemon");
                let stdout = File::create("/tmp/hermod.out").unwrap();
                let stderr = File::create("/tmp/hermod.err").unwrap();
                let daemon = Daemonize::new()
                    .pid_file("/tmp/hermod.pid")
                    .working_directory(dirs::home_dir().expect("Could not find home directory"))
                    .stdout(stdout)
                    .stderr(stderr);

                match daemon.start() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error: Failed to daemonize server: ({}).\n Aborting...", e);
                        return;
                    }
                }

                match hermod::log::setup_logger(false, verbosity) {
                    Ok(()) => (),
                    Err(e) => eprintln!("Failed to initate logging, aborting. ({})", e),
                }
            } else {
                std::env::set_current_dir(dirs::home_dir().expect("Failed to read home directory"))
                    .expect("Failed to set current working directory");

                match hermod::log::setup_logger(true, verbosity) {
                    Ok(()) => (),
                    Err(e) => eprintln!("Failed to initate logging, aborting. ({})", e),
                }
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
    let source = args
        .value_of("source")
        .expect("Obligatory argument 'source' missing, aborting");
    let destination = args
        .value_of("destination")
        .expect("Obligatory argument 'destination' missing, aborting");
    let compression = args.is_present("compression");

    let cfg_builder = ClientConfigBuilder::new(&host)
        .source(source)
        .destination(destination)
        .compression(compression)
        .request(method);

    let cfg = cfg_builder.build_config();

    hermod::client::HermodClient::new(cfg).execute();
}

fn gen_key(args: &clap::ArgMatches) {
    println!("Generating a new static keypair and a new identification token...");
    let alias = args.value_of("alias").expect("No alias provided, aborting");
    let force = args.is_present("force");

    if hermod::host::exists(&alias) && !force {
        eprintln!("Found an existing host with that alias, to overwrite pass --force");
        return;
    } else {
        println!("Found an existing host with that alias, overwriting");
    }

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
    let name = args.value_of("name").expect("No naem provided, aborting");
    let force = args.is_present("force");

    if hermod::host::exists(&name) && !force {
        eprintln!("Found an existing host with that alias, to overwrite pass --force");
        return;
    } else {
        println!("Found an existing host with that alias, overwriting");
    }

    let host = args
        .value_of("host")
        .expect("No host address provided, aborting");
    let host = hermod::host::Host::with_alias(&name).set_hostname(host);
    hermod::share_key::share_key(host);
}
