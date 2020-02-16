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
        ("gen-key", Some(_)) => gen_key(),
        ("share-key", Some(sk_args)) => share_key(&sk_args),
        _ => {}
    }
}

fn start_server(args: &clap::ArgMatches) {
    match args.subcommand() {
        ("init", Some(_)) => {
            hermod::genkey::gen_server_keys().unwrap();
            return;
        }
        ("list", Some(_)) => {
            HermodServer::list_known_clients();
            return;
        }
        _ => {}
    }
    async_std::task::block_on(async {
        HermodServer::run_server().await;
    });
}

fn exec_request(args: &clap::ArgMatches, method: RequestMethod) {
    let host = hermod::host::load_host(args.value_of("remote").unwrap()).unwrap();
    let source = args
        .value_of("source")
        .expect("Obligatory argument 'source' missing");
    let destination = args
        .value_of("destination")
        .expect("Obligatory argument 'destination' missing");
    let compression = args.is_present("compression");
    let cfg = ClientConfigBuilder::new(&host)
        .source(source)
        .destination(destination)
        .compression(compression)
        .request(method)
        .build_config();
    async_std::task::block_on(async {});
}

fn gen_key() {
    let keys = hermod::genkey::gen_keys().unwrap();
    let id_token = 5;
}

fn share_key(args: &clap::ArgMatches) {
    unimplemented!()
}
