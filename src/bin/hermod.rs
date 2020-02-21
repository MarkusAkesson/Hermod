use hermod::cli;
use hermod::config::ClientConfigBuilder;
use hermod::request::RequestMethod;
use hermod::server::HermodServer;

use std::fs::File;

use daemonize::Daemonize;

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
        ("setup", Some(_)) => {
            println!("Genereting new static files for the server...");
            HermodServer::setup();
        }
        ("list", Some(_)) => {
            HermodServer::list_known_clients();
        }
        // Treat all other cases as wanting to run the server
        _ => {
            if !args.is_present("no-daemon") {
                println!("Preparing to run server as a daemon");
                let stdout = File::create("/tmp/hermod.out").unwrap();
                let stderr = File::create("/tmp/hermod.err").unwrap();
                let daemon = Daemonize::new()
                    .pid_file("/tmp/hermod.pid")
                    .stdout(stdout)
                    .stderr(stderr);

                match daemon.start() {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error: Failed to daemonize server: ({}).\n Aborting...", e);
                        return;
                    }
                }
            }
            println!("Preparing to run the server");
            HermodServer::run_server();
        }
    }
}

fn exec_request(args: &clap::ArgMatches, method: RequestMethod) {
    let host = match hermod::host::load_host(args.value_of("remote").unwrap()) {
        Ok(host) => host,
        Err(err) => {
            eprintln!("{}", err);
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
    let keys = hermod::genkey::gen_keys().unwrap();
    let private_key = keys.private;
    let public_key = keys.public;
    let id_token = hermod::genkey::gen_idtoken();

    let alias = args.value_of("alias").expect("No alias provided, aborting");

    let host = hermod::host::Host::with_alias(&alias)
        .set_id_token(&id_token)
        .set_public_key(&public_key)
        .set_private_key(&private_key);

    println!("{}", host);

    host.write_to_file().unwrap();
}

fn share_key(args: &clap::ArgMatches) {
    unimplemented!()
}
