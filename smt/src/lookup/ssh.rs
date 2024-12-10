
use ssh2::{Session, Channel};
use std::io::Read;
use std::net::TcpStream;
use log::{info, warn, error};

use crate::structs::lookup::Lookup;

use crate::constants::{
    WARN_SKIPPING_HANDSHAKE,
    WARN_SKIPPING_AUTHENTICATION,
    WARN_SKIPPING_CHANNEL_CREATION,
    WARN_SKIPPING_CHANNEL_CLOSURE,
    WARN_SKIPPING_CHANNEL_WAIT_CLOSURE,
    WARN_SKIPPING_DISCONNECTION,
    ERROR_CONNECTION_FAILED,
    INFO_CONNECTION_SUCCESSFUL,
    ERROR_HANDSHAKE_FAILED,
    INFO_HANDSHAKE_SUCCESSFUL,
    ERROR_AUTHENTICATION_FAILED,
    INFO_AUTHENTICATION_SUCCESSFUL,
    ERROR_CHANNEL_CREATION_FAILED,
    INFO_CHANNEL_CREATION_SUCCESSFUL,
    ERROR_CHANNEL_CLOSURE_FAILED,
    INFO_CHANNEL_CLOSURE_SUCCESSFUL,
    ERROR_CHANNEL_WAIT_CLOSURE_FAILED,
    INFO_CHANNEL_WAIT_CLOSURE_SUCCESSFUL,
    ERROR_DISCONNECTION_FAILED,
    INFO_DISCONNECTION_SUCCESSFUL,
    ERROR_BANNER_RETRIEVE_FAILED,
    INFO_BANNER_RETRIEVE_SUCCESSFUL,
    WARN_SKIPPING_BANNER_RETRIEVE,
    WARN_SKIPPING_ENV_RETRIEVE,
    ERROR_ENV_RETRIEVE_FAILED,
    ERROR_ENV_READING_FAILED,
    INFO_ENV_RETRIEVE_SUCCESSFUL
};

#[derive(PartialEq, Debug)]
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
    FailedChannelCreation,
    SuccessChannelClosure,
    FailedChannelClosure,
}

pub struct Ssh {
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: u32,

    pub lookup: Lookup,

    session: Session,
    pub session_state: SessionStates,
    session_channel: Option<Channel>,

    verbose: bool,
}

impl Ssh {
    pub fn new(username: String, password: String, address: String, port: u32, verbose: bool) -> Ssh {
        Ssh {
            username,
            password,
            address,
            port,
            session: Session::new().unwrap(),
            session_channel: None,
            session_state: SessionStates::Disconnected,
            verbose,
            lookup: Lookup {
                server_env_variables: None,
                server_ssh_banner: None
            }
        }
    }

    fn verbose_log(&self, message: &str) -> () {
        if self.verbose {
            warn!("{}", message);
        }
    }

    pub fn connect(&mut self) -> () {
        self.establish_connection();
        self.perform_handshake();
        self.authenticate();
    }

    pub fn scan(&mut self) -> () {
        self.retrieve_banner();
        self.create_channel();
        self.retrieve_env();
        self.close_channel();
        self.wait_closure();
    }

    pub fn establish_connection(&mut self) -> () {
        let tcp_stream: Result<TcpStream, std::io::Error> = TcpStream::connect(
            format!("{}:{}", self.address, self.port)
        );

        if self.session_state == SessionStates::SuccessConnection {
            self.disconnect();
        }

        if let Err(e) = tcp_stream {
            error!("{}: {:?}", ERROR_CONNECTION_FAILED, e);
            self.session_state = SessionStates::FailedConnection;
        } else if let Ok(tcp) = tcp_stream {
            self.session.set_tcp_stream(tcp);
            info!("{}", INFO_CONNECTION_SUCCESSFUL);
            self.session_state = SessionStates::SuccessConnection;
        }
    }

    pub fn perform_handshake(&mut self) -> () {
        let states: Vec<SessionStates> = vec![
            SessionStates::FailedHandshake,
            SessionStates::SuccessConnection
        ];

        if states.contains(&self.session_state) {
            if let Err(e) = self.session.handshake() {
                error!("{}: {:?}", ERROR_HANDSHAKE_FAILED, e);
                self.session_state = SessionStates::FailedHandshake;
            } else {
                info!("{}", INFO_HANDSHAKE_SUCCESSFUL);
                self.session_state = SessionStates::SuccessHandshake;
            }
        } else {
            self.verbose_log(WARN_SKIPPING_HANDSHAKE);
        }
    }

    pub fn authenticate(&mut self) -> () {
        let states: Vec<SessionStates> = vec![
            SessionStates::FailedAuthentication,
            SessionStates::SuccessChannelCreation,
            SessionStates::SuccessHandshake,
        ];

        if states.contains(&self.session_state) {
            if let Err(e) = self.session.userauth_password(&self.username, &self.password) {
                error!("{}: {:?}", ERROR_AUTHENTICATION_FAILED, e);
                self.session_state = SessionStates::FailedAuthentication;
            } else {
                info!("{}", INFO_AUTHENTICATION_SUCCESSFUL);
                self.session_state = SessionStates::SuccessAuthentication;
            }
        } else {
            self.verbose_log(WARN_SKIPPING_AUTHENTICATION);
        }
    }

    fn create_channel(&mut self) -> () {
        let channel_creation: Result<Channel, ssh2::Error> = self.session.channel_session();
        let states: Vec<SessionStates> = vec![
            SessionStates::SuccessAuthentication,
            SessionStates::FailedChannelCreation
        ];

        if states.contains(&self.session_state) {
            if let Err(e) = channel_creation {
                error!("{}: {:?}", ERROR_CHANNEL_CREATION_FAILED, e);
                self.session_state = SessionStates::FailedChannelCreation;
            } else if let Ok(channel) = channel_creation {
                info!("{}", INFO_CHANNEL_CREATION_SUCCESSFUL);
                self.session_channel = Some(channel);
                self.session_state = SessionStates::SuccessChannelCreation;
            }
        } else {
            self.verbose_log(WARN_SKIPPING_CHANNEL_CREATION);
        }
    }

    fn close_channel(&mut self) -> () {
        let states: Vec<SessionStates> = vec![
            SessionStates::FailedChannelClosure,
            SessionStates::SuccessChannelCreation,
        ];

        if states.contains(&self.session_state) {
            if let Some(channel) = self.session_channel.as_mut() {
                if let Err(e) = channel.close() {
                    error!("{}: {:?}", ERROR_CHANNEL_CLOSURE_FAILED, e);
                    self.session_state = SessionStates::FailedChannelClosure;
                } else {
                    info!("{}", INFO_CHANNEL_CLOSURE_SUCCESSFUL);
                    self.session_state = SessionStates::SuccessChannelClosure;
                }
            }
        } else {
            self.verbose_log(WARN_SKIPPING_CHANNEL_CLOSURE);
        }
    }

    fn wait_closure(&mut self) -> () {
        let states: Vec<SessionStates> = vec![
            SessionStates::SuccessChannelClosure
        ];

        if states.contains(&self.session_state) {
            if let Some(channel) = self.session_channel.as_mut() {
                if let Err(e) = channel.wait_close() {
                    error!("{}: {:?}", ERROR_CHANNEL_WAIT_CLOSURE_FAILED, e);
                } else {
                    info!("{}", INFO_CHANNEL_WAIT_CLOSURE_SUCCESSFUL);
                    self.session_channel = None;
                }
            }
        } else {
            self.verbose_log(WARN_SKIPPING_CHANNEL_WAIT_CLOSURE);
        }
    }

    pub fn disconnect(&mut self) -> () {
        let states: Vec<SessionStates> = vec![
            SessionStates::SuccessAuthentication,
            SessionStates::SuccessChannelCreation,
            SessionStates::SuccessChannelClosure,
            SessionStates::SuccessConnection,
            SessionStates::SuccessHandshake,
            SessionStates::FailedAuthentication,
            SessionStates::FailedChannelClosure,
            SessionStates::FailedChannelCreation,
            SessionStates::FailedHandshake
        ];

        if states.contains(&self.session_state) {
            if let Err(e) = self.session.disconnect(None, "", None) {
                error!("{}: {:?}", ERROR_DISCONNECTION_FAILED, e);
                self.session_state = SessionStates::FailedDisconnection;
            } else {
                info!("{}", INFO_DISCONNECTION_SUCCESSFUL);
                self.session_state = SessionStates::SuccessDisconnection;
            }
        } else {
            self.verbose_log(WARN_SKIPPING_DISCONNECTION);
        }
    }

    fn retrieve_banner(&mut self) -> () {
        if self.session_state == SessionStates::SuccessAuthentication {
            if let Some(banner) = self.session.banner() {
                self.lookup.server_ssh_banner = Some(banner.to_owned());
                info!("{}", INFO_BANNER_RETRIEVE_SUCCESSFUL);
            } else {
                error!("{}", ERROR_BANNER_RETRIEVE_FAILED);
            }
        } else {
            self.verbose_log(WARN_SKIPPING_BANNER_RETRIEVE);
        }
    }

    fn retrieve_env(&mut self) -> () {
        let mut output = String::new();
        let states: Vec<SessionStates> = vec![
            SessionStates::SuccessChannelCreation,
        ];
    
        if states.contains(&self.session_state) {
            if let Some(channel) = &mut self.session_channel {
                if let Err(e) = channel.exec("env") {
                    error!("{}: {:?}", ERROR_ENV_RETRIEVE_FAILED, e);
                } else {
                    if let Err(e) = channel.read_to_string(&mut output) {
                        error!("{}: {:?}", ERROR_ENV_READING_FAILED, e);
                    } else {
                        self.lookup.server_env_variables = Some(output);
                        info!("{}", INFO_ENV_RETRIEVE_SUCCESSFUL);
                    }
                }
            }
        } else {
            self.verbose_log(WARN_SKIPPING_ENV_RETRIEVE);
        }
    }
}
