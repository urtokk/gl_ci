/// Program to setup a Gitlab projects CI env variables in the local shell.
/// This is useful for local development and testing of CI scripts.
/// The program will read the CI variables from the Gitlab API and set them in the local shell.
/// It needs the GITLAB_TOKEN environment variable to be set.

use std::env;
use clap::Parser;

mod gitlab;

#[derive(Debug,Parser)]
#[command(author, about, version, long_about=None)]
struct Args {
    #[arg(long, short, default_value = "https://gitlab.com")]
    server: String,
    #[arg(long, short)]
    project: String,
    #[arg(long, short)]
    token: String,
    #[arg(long, short)]
    env: String,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    // Setup logging
    let args = Args::parse();
    let gl_token = match env::var("GITLAB_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("GITLAB_TOKEN is not set"),
    };
}
