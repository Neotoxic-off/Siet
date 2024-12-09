use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments
{
    #[arg(long, required = true)]
    pub host: String,

    #[arg(long, required = true)]
    pub port: u32,

    #[arg(long, required = true)]
    pub username: String,

    #[arg(long, required = true)]
    pub passwords: String,

    #[arg(short, long, required = false)]
    pub verbose: bool
}
