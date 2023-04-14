use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn read_stream_lines(stream: &TcpStream) -> Result<Vec<String>, Box<dyn Error>> {
    let buf_reader = BufReader::new(stream);
    let mut lines: Vec<String> = Vec::new();
    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    break;
                };
                lines.push(line);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(lines)
}
