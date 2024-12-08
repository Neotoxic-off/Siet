use ssh2::Session;
use std::format;
use log::{info, warn, error};
use std::net::TcpStream;

pub struct Ssh
{
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: u32,

    session: Session
}

impl Ssh {
    pub fn new(username: String, password: String, address: String, port: u32) -> Ssh {
        Ssh {
            username,
            password,
            address,
            port,
            session: Session::new().unwrap()
        }
    }

    pub fn connect(&mut self) -> () {
        match TcpStream::connect(format!("{}:{}", self.address, self.port)) {
            Ok(tcp) => {
                self.session.set_tcp_stream(tcp);
                match self.session.handshake() {
                    Ok(_) => {
                        match self.session.userauth_password(&self.username, &self.password) {
                            Ok(_) => {
                                if self.session.authenticated() {
                                    info!("Authentication successful!");
                                } else {
                                    error!("Authentication failed!");
                                }
                            }
                            Err(e) => {
                                error!("Authentication error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("SSH handshake failed: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Could not connect to server: {:?}", e);
            }
        }

    }
    
    pub fn disconnect(&self) -> () {
        self.session.disconnect(None, "", None);
    }
}
