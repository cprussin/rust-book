use crate::{thread_pool::ThreadPool, Headers, Request, Response, Status};
use std::{
    io,
    net::{Shutdown, TcpListener, TcpStream},
};

pub struct Server<'a> {
    pub listen: &'a str,
    pub port: u16,
    pub pool_size: usize,
}

impl<'a> Server<'a> {
    pub fn serve<T>(&self, router: T) -> io::Result<()>
    where
        T: FnMut(&Request) -> Response,
        T: Copy + Send + 'static,
    {
        let pool = ThreadPool::new(self.pool_size);
        let listener = TcpListener::bind(format!("{}:{}", self.listen, self.port))?;
        println!("Server up on {}:{}", self.listen, self.port);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => pool.execute(move || handle_request(stream, router)),
                Err(err) => eprintln!("{err}"),
            }
        }
        Ok(())
    }
}

fn handle_request<T>(stream: TcpStream, router: T)
where
    T: FnOnce(&Request) -> Response,
{
    try_handle_request(stream, router).unwrap_or_else(|err| eprintln!("{err}"))
}

fn try_handle_request<T>(mut stream: TcpStream, router: T) -> io::Result<()>
where
    T: FnOnce(&Request) -> Response,
{
    let response = match Request::parse(&mut stream) {
        Ok(request) => router(&request),
        _ => Response {
            status: Status::BadRequest,
            headers: Headers::empty(),
            body: None,
        },
    };
    response.write(&mut stream)?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
