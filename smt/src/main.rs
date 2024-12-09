use env_logger;
use clap::Parser;
use log::{info, log};
use lookup::ssh::SessionStates;

pub mod io;
pub mod lookup;
pub mod structs;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn load(file: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    if io::File::exists(&file) == true {
        lines = io::File::read_lines(&file);
    }

    lines
}

fn run(username: String, host: String, port: u32, lines: Vec<String>) -> () {
    let mut ssh: lookup::ssh::Ssh = lookup::ssh::Ssh::new(
        String::from(username),
        String::from("default"),
        String::from(host),
        port
    );

    info!("({}@{}:{}):{}", ssh.username, ssh.address, port, lines.len());

    ssh.establish_connection();
    ssh.perform_handshake();
    for line in lines.iter() {
        ssh.password = line.to_owned();
        info!("{}", line);
        ssh.authenticate();
        if ssh.session_state == SessionStates::SuccessAuthentication {
            ssh.lookup();
            info!("Password found: '{}'", line.to_owned());
            ssh.disconnect();
            return;
        }
    }
}

fn main() {
    let arguments: structs::arguments::Arguments = structs::arguments::Arguments::parse();
    let mut lines: Vec<String> = Vec::new();

    setup();

    lines = load(arguments.passwords);
    run(arguments.username, arguments.host, arguments.port, lines);
}
