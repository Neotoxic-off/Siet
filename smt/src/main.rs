use env_logger;
pub mod protocols;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn main() {
    setup();

    let mut ssh: protocols::ssh::Ssh = protocols::ssh::Ssh::new(
        String::from("root"),
        String::from("root"),
        String::from("127.0.0.1"),
        22
    );

    ssh.connect();
}
