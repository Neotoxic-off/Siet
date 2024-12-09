use env_logger;
use clap::Parser;

pub mod lookup;
pub mod structs;
pub mod constants;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn connect(username: String, host: String, port: u32, password: String, verbose: bool) -> () {
    let mut ssh: lookup::ssh::Ssh = lookup::ssh::Ssh::new(
        String::from(username),
        String::from(password),
        String::from(host),
        port,
        verbose
    );

    ssh.connect();
    ssh.lookup();
    ssh.disconnect();
}

fn main() {
    let arguments: structs::arguments::Arguments = structs::arguments::Arguments::parse();

    setup();

    connect(arguments.username, arguments.host, arguments.port, arguments.password, arguments.verbose);
}
