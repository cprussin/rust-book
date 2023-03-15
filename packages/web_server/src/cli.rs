use crate::{Headers, Request, Response, Server, Status};
use clap::Parser;
use std::{process::ExitCode, thread, time::Duration};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    listen: String,

    #[arg(long, default_value_t = 8)]
    pool_size: usize,
}

pub fn main() -> ExitCode {
    let cli = Cli::parse();
    let server = Server {
        listen: &cli.listen,
        port: cli.port,
        pool_size: cli.pool_size,
    };
    match server.serve(router) {
        Ok(_) => ExitCode::from(0),
        Err(err) => {
            eprintln!("Error creating server: {err}");
            ExitCode::from(1)
        }
    }
}

fn router(Request { target, .. }: &Request) -> Response {
    match target.as_str() {
        "/" => Response {
            status: Status::Ok,
            headers: Headers::empty(),
            body: Some(String::from("<html><head><title>Hello World!</title></head><body><h1>Why hello!</h1></body></html>"))
        },

        "/slow" => {
            thread::sleep(Duration::from_secs(5));
            Response {
                status: Status::Ok,
                headers: Headers::empty(),
                body: Some(String::from("<html><head><title>This is slow!</title></head><body><h1>Sooo slow</h1></body></html>"))
            }
        }

        _ => Response {
            status: Status::NotFound,
            headers: Headers::empty(),
            body: None
        }
    }
}
