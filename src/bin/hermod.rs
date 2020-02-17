use hermod::cli;
use hermod::config::{ClientConfig, ClientConfigBuilder, ServerConfig};
use hermod::request::RequestMethod;
use hermod::server::HermodServer;

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
        ("init", Some(_)) => {
            hermod::genkey::gen_server_keys().unwrap();
        }
        ("list", Some(_)) => {
            HermodServer::list_known_clients();
        }
        _ => {
            // Treat all other cases as wanting to run the server
            // TODO: make nicer
            HermodServer::run_server();
        }
    }
}

fn exec_request(args: &clap::ArgMatches, method: RequestMethod) {
    let host = hermod::host::load_host(args.value_of("remote").unwrap()).unwrap();
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
    let keys = hermod::genkey::gen_keys().unwrap();
    let private_key = keys.private;
    let public_key = keys.public;
    let id_token = String::new();

    let alias = args.value_of("alias").unwrap();

    let host = hermod::host::Host::with_alias(&alias)
        .set_id_token(&id_token)
        .set_public_key(&public_key)
        .set_private_key(&private_key);

    println!("{}", host);

    if host.hostname().is_empty() {
        return;
    }
    host.write_to_file().unwrap();
}

fn share_key(args: &clap::ArgMatches) {
    unimplemented!()
}
