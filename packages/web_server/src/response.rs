use crate::{Headers, Status};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub headers: Headers,
    pub body: Option<String>,
}

impl Response {
    pub fn write<T: Write>(&self, w: &mut T) -> io::Result<()> {
        w.write_all(format!("HTTP/1.1 {} {}\n", self.status.code(), self.status).as_bytes())?;
        for (key, value) in self.headers.iter() {
            w.write_all(format!("{key}: {value}\n").as_bytes())?;
        }
        if let Some(body) = &self.body {
            let bytes = body.as_bytes();
            w.write_all(b"Content-Type: text/html; charset=utf-8\n")?;
            w.write_all(format!("Content-Length: {}\n", bytes.len()).as_bytes())?;
            w.write_all(b"\n")?;
            w.write_all(bytes)?;
            w.write_all(b"\n")?;
        }
        w.flush()?;
        Ok(())
    }
}
