use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_matches() -> clap::ArgMatches<'static> {
    App::new("Hermod File Transefer util")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Hermod is a file transfer util that utilises the Hermod file transfer protocol")
        .subcommand(SubCommand::with_name("server")
            .about("Start a server")
            .arg(Arg::with_name("no-daemon")
                .long("no-daemon")
                .help("Do not run the server in the background"))
            .arg(Arg::with_name("ip")
                .long("ip")
                .takes_value(true)
                .help("IP address to bind to")
                .default_value("0.0.0.0"))
            .subcommand(SubCommand::with_name("setup")
                .about("Generate static keys for the server")
                .arg(Arg::with_name("force")
                    .long("force")
                    .help("Overwrite existing keys if found")))
            .subcommand(SubCommand::with_name("list")
                .about("List all authorized client")))
        .subcommand(SubCommand::with_name("gen-key")
            .about("Generate static keys for the client and a new client-token")
            .arg(Arg::with_name("force")
                .long("force")
                .help("Overwrite existing keys if found"))
            .arg(Arg::with_name("alias")
                .required(true)
                .takes_value(true)
                .long("alias")
                .help("Alias us use for the remote server")))
        .subcommand(SubCommand::with_name("share-key")
            .about("Generate and immediately share keys and id with the specified host")
            .arg(Arg::with_name("force")
                .long("force")
                .help("Overwrite existing keys if found"))
            .arg(Arg::with_name("host")
                .long("host")
                .short("h")
                .value_name("HOST")
                .takes_value(true)
                .required(true)
                .help("Host address of the server to share keys with"))
            .arg(Arg::with_name("name")
                .long("name")
                .short("n")
                .value_name("NAME")
                .takes_value(true)
                .required(true)
                .help("The remote hostname or ip address for the server to share a public keys with. Generates new client keys for the server if they dont exists")))
        .subcommand(SubCommand::with_name("upload")
            .about("Upload a file or files to the remote server")
            .arg(Arg::with_name("remote")
                .long("remote")
                .short("r")
                .value_name("REMOTE")
                .takes_value(true)
                .required(true)
                .help("The alias for the remote server to upload files to."))
            .arg(Arg::with_name("destination")
                .long("destination")
                .short("d")
                .value_name("DESTINATION")
                .takes_value(true)
                .required(true)
                .help("Destination folder for the transmitted files"))
            .arg(Arg::with_name("source")
                .long("source")
                .short("s")
                .value_name("SOURCE")
                .takes_value(true)
                .required(true)
                .help("The source file or files to send to the server")))
        .subcommand(SubCommand::with_name("download")
            .about("Download a file or files from the remote server")
            .arg(Arg::with_name("remote")
                .long("remote")
                .short("r")
                .value_name("REMOTE")
                .takes_value(true)
                .required(true)
                .help("The alias for the remote server to download files from."))
            .arg(Arg::with_name("destination")
                .long("destination")
                .short("d")
                .value_name("DESTINATION")
                .takes_value(true)
                .required(true)
                .help("Destination folder for the transmitted files"))
            .arg(Arg::with_name("source")
                .long("source")
                .short("s")
                .value_name("SOURCE")
                .takes_value(true)
                .required(true)
                .help("The source file or files to downlaod from the server"))).get_matches()
}
