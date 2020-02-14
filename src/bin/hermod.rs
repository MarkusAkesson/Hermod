use hermod::cli;
use hermod::config::{ClientConfig, ServerConfig};
use hermod::server::HermodServer;

use async_std::sync::Arc;

fn main() {
    let args = cli::get_matches();

    match args.subcommand() {
        ("server", Some(server_args)) => start_server(&server_args),
        ("upload", Some(req_args)) | ("download", Some(req_args)) => exec_request(&req_args),
        ("gen-key", None) => gen_key(),
        ("share-key", Some(sk_args)) => share_key(&sk_args),
        _ => {}
    }
}

fn start_server(args: &clap::ArgMatches) {
    let cfg = Arc::new(ServerConfig::new());
    match args.subcommand() {
        ("init", Some(init_args)) => {
            hermod::genkey::gen_server_keys().unwrap();
            return;
        }
        ("list", None) => {
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
    unimplemented!()
}

fn gen_key() {
    unimplemented!()
}

fn share_key(args: &clap::ArgMatches) {
    unimplemented!()
}
