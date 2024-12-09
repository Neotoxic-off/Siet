use std::format;
use ssh2::{Session, Channel};
use std::path::Path;
use std::io::{Read, Write};
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
    FailedDisconnection,
    SuccessChannelCreation,
    FailedChannelCreation
}

pub struct Ssh
{
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: u32,

    pub server_ssh_banner: String,

    session: Session,
    pub session_state: SessionStates,
    session_channel: Option<Channel>
}

impl Ssh {
    pub fn new(username: String, password: String, address: String, port: u32) -> Ssh {
        Ssh {
            username,
            password,
            address,
            port,
            server_ssh_banner: String::new(),
            session: Session::new().unwrap(),
            session_channel: None,
            session_state: SessionStates::Disconnected
        }
    }

    pub fn connect(&mut self) -> () {
        self.establish_connection();
        self.perform_handshake();
        self.authenticate();
    }

    pub fn update(&mut self, username: String, password: String, address: String, port: u32) -> () {
        self.username = username;
        self.password = password;
        self.address = address;
        self.port = port;
    }

    pub fn lookup(&mut self) -> () {
        self.retrieve_banner();
        self.create_channel();
        self.close_channel();
    }

    pub fn establish_connection(&mut self) -> () {
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

    pub fn perform_handshake(&mut self) -> () {
        let states: Vec<SessionStates> = Vec::from([
            SessionStates::FailedHandshake,
            SessionStates::SuccessConnection
        ]);

        if states.contains(&self.session_state) == true {
            if let Err(e) = self.session.handshake() {
                error!("Handshake failed: {:?}", e);
                self.session_state = SessionStates::FailedHandshake;
            } else {
                info!("Handshake successful");
                self.session_state = SessionStates::SuccessHandshake;
            }
        } else {
            warn!("Skipping handshake");
        }
    }

    pub fn authenticate(&mut self) -> () {
        let states: Vec<SessionStates> = Vec::from([
            SessionStates::FailedAuthentication, SessionStates::SuccessChannelCreation,
            SessionStates::SuccessHandshake
        ]);

        if states.contains(&self.session_state) == true {
            if let Err(e) = self.session.userauth_password(&self.username, &self.password) {
                error!("Authentication failed: {:?}", e);
                self.session_state = SessionStates::FailedAuthentication;
            } else {
                info!("Authentication successful");
                self.session_state = SessionStates::SuccessAuthentication;
            }
        } else {
            warn!("Skipping authentication");
        }
    }

    fn create_channel(&mut self) -> () {
        let states: Vec<SessionStates> = Vec::from([
            SessionStates::SuccessAuthentication,
            SessionStates::SuccessChannelCreation
        ]);

        if states.contains(&self.session_state) == true {
            if self.session_channel.is_none() {
                if let Err(e) = self.session.channel_session() {
                    error!("Channel creation failed: {:?}", e);
                    self.session_state = SessionStates::FailedChannelCreation;
                } else {
                    info!("Channel creation successful");
                    self.session_state = SessionStates::SuccessChannelCreation;
                }
            }
        } else {
            warn!("Skipping channel creation");
        }
    }

    fn close_channel(&mut self) -> () {
        if self.session_channel.is_some() {
            if let Some(channel) = self.session_channel.as_mut() {
                match channel.close() {
                    Ok(_) => info!("Channel closed successfully"),
                    Err(e) => {
                        error!("Failed to close channel: {:?}", e);
                        return;
                    }
                }
    
                match channel.wait_close() {
                    Ok(_) => {
                        info!("Channel wait_close completed successfully");
                        self.session_channel = None;
                    }
                    Err(e) => {
                        error!("Failed to complete channel wait_close: {:?}", e);
                    }
                }
            }
        } else {
            warn!("Skipping channel closure");
        }
    }
    

    pub fn disconnect(&mut self) -> () {
        let states: Vec<SessionStates> = Vec::from([
            SessionStates::SuccessAuthentication, SessionStates::SuccessChannelCreation,
            SessionStates::SuccessConnection, SessionStates::SuccessHandshake
        ]);

        if states.contains(&self.session_state) == true {
            if let Err(e) = self.session.disconnect(None, "", None) {
                error!("Disconnection failed: {:?}", e);
                self.session_state = SessionStates::FailedDisconnection;
            } else {
                info!("Disconnection successful");
                self.session_state = SessionStates::SuccessDisconnection;
            }
        } else {
            warn!("Skipping disconnection");
        }
    }

    fn retrieve_banner(&mut self) -> () {
        if self.session_state == SessionStates::SuccessAuthentication {
            if let Some(banner) = self.session.banner() {
                self.server_ssh_banner = banner.to_owned();
                info!("SSH banner retrieved: {}", banner)
            } else {
                error!("SSH banner retrieve failed");
            }
        } else {
            warn!("Skipping banner retrive");
        }
    }
}
