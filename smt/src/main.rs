use env_logger;
pub mod lookup;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn main() {
    setup();

    let mut ssh: lookup::ssh::Ssh = lookup::ssh::Ssh::new(
        String::from("test"),
        String::from("test"),
        String::from("192.168.0.0"),
        22
    );

    ssh.connect();
    ssh.lookup();
    ssh.disconnect();
}
