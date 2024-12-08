use ssh2::Session;
use std::format;
use log::{info, warn, error};
use std::net::TcpStream;

#[derive(PartialEq)]
pub enum SessionStates {
    Disconnected,
    SuccessConnection,
    FailedConnection,
    SuccessHandshake,
    FailedHandshake,
    SuccessAuthentication,
    FailedAuthentication,
    SuccessDisconnection,
    FailedDisconnection
}

pub struct Ssh
{
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: u32,

    session: Session,
    session_state: SessionStates
}

impl Ssh {
    pub fn new(username: String, password: String, address: String, port: u32) -> Ssh {
        Ssh {
            username,
            password,
            address,
            port,
            session: Session::new().unwrap(),
            session_state: SessionStates::Disconnected
        }
    }

    pub fn connect(&mut self) -> () {
        self.establish_connection();
        self.perform_handshake();
        self.authenticate();
    }

    fn establish_connection(&mut self) -> () {
        if self.session_state == SessionStates::SuccessConnection {
            self.disconnect();
        }
 
        match TcpStream::connect(format!("{}:{}", self.address, self.port)) {
            Ok(tcp) => {
                self.session.set_tcp_stream(tcp);
                info!("Connection successful");
                self.session_state = SessionStates::SuccessConnection;
            }
            Err(e) => {
                error!("Connection failed: {:?}", e);
                self.session_state = SessionStates::FailedConnection;
            }
        }
    }    

    fn perform_handshake(&mut self) -> () {
        if self.session_state == SessionStates::SuccessConnection {
            if let Err(e) = self.session.handshake() {
                error!("Handshake failed: {:?}", e);
                self.session_state = SessionStates::FailedHandshake;
            } else {
                info!("Handshake successful");
                self.session_state = SessionStates::SuccessHandshake;
            }
        } else {
            warn!("Session failed connection, skipping handshake");
        }
    }

    fn authenticate(&mut self) -> () {
        if self.session_state == SessionStates::SuccessHandshake {
            if let Err(e) = self.session.userauth_password(&self.username, &self.password) {
                error!("Authentication failed: {:?}", e);
                self.session_state = SessionStates::FailedAuthentication;
            } else {
                info!("Authentication successful");
                self.session_state = SessionStates::SuccessAuthentication;
            }
        } else {
            warn!("Session failed handshake, skipping authentication");
        }
    }

    pub fn disconnect(&mut self) -> () {
        if self.session_state == SessionStates::SuccessConnection {
            if let Err(e) = self.session.disconnect(None, "", None) {
                error!("Disconnection failed: {:?}", e);
                self.session_state = SessionStates::FailedDisconnection;
            } else {
                info!("Disconnection successful");
                self.session_state = SessionStates::SuccessDisconnection;
            }
        } else {
            warn!("Session failed connection, skipping disconnection");
        }
    }
}
