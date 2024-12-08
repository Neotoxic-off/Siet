pub mod protocol;

fn main() {
    let mut ssh: protocol::ssh::Ssh = protocol::ssh::Ssh::new(
        String::from("root"),
        String::from("root"),
        String::from("127.0.0.1"),
        22
    );

    ssh.connect();
}
