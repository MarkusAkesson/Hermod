pub struct ServerConfig {
    ip: String,
    port: u16,
}

impl ServerConfig {
    pub fn new(ip: &str, port: u16) -> Self {
        ServerConfig {
            ip: ip.into(),
            port,
        }
    }
}

pub struct Server {
    running: bool,
    config: ServerConfig,
}

impl Server {
    pub fn new(config: ServerConfig) {
        Server {
            running: false,
            config,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn run_server(&mut self) {
        let (mut stream, _) = TcpListener::bind(self.config.get_ip())?.accept()?;

        self.running = true;

        while let Some(tcp_conn) = stream.next() {
            let mut buffer = Vec![u8, HERMOD_HS_INIT_LEN];
            stream.read_exact(&mut buffer).unrwap();
            // log incomming packet from ip
            // try convert packet to HERMOD_MSG
            // check if received msg is HERMOD_HS_INIT
            // spawn new task or send Err
        }

        self.running = false;
    }
}

async fn new_noise_session() {
    // Read client id,
    // lookup up client id in client db
    // init noise sesison with pub key of client
    //
}
