use hermod::cli;
use hermod::config::{ClientConfig, ClientConfigBuilder, ServerConfig};
use hermod::server::HermodServer;

fn main() {
    let args = cli::get_matches();

    match args.subcommand() {
        ("server", Some(server_args)) => start_server(&server_args),
        ("upload", Some(req_args)) | ("download", Some(req_args)) => exec_request(&req_args),
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

fn exec_request(args: &clap::ArgMatches) {
    let host = hermod::host::load_host(args.value_of("remote").unwrap()).unwrap();
    let cfg = ClientConfigBuilder::new(&host);
    unimplemented!()
}

fn gen_key() {
    let keys = hermod::genkey::gen_keys().unwrap();
    let id_token = 5;
}

fn share_key(args: &clap::ArgMatches) {
    unimplemented!()
}
