use env_logger;
use clap::Parser;

pub mod lookup;
pub mod structs;
pub mod report;
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
    ssh.scan();
    ssh.disconnect();

    build_report(ssh);
}

fn build_report(ssh: lookup::ssh::Ssh) -> () {
    if let Some(ssh_banner) = ssh.lookup.server_ssh_banner {
        report::save("report/ssh_banner.txt", &ssh_banner).unwrap();
    }

    if let Some(env_variables) = ssh.lookup.server_env_variables {
        report::save("report/env_variables.txt", &env_variables).unwrap();
    }

    if let Some(bashrc) = ssh.lookup.server_bashrc {
        report::save("report/bashrc.txt", &bashrc).unwrap();
    }

    if let Some(bash_history) = ssh.lookup.server_bash_history {
        report::save("report/bash_history.txt", &bash_history).unwrap();
    }
}

fn main() {
    let arguments: structs::arguments::Arguments = structs::arguments::Arguments::parse();

    setup();

    connect(arguments.username, arguments.host, arguments.port, arguments.password, arguments.verbose);
}
