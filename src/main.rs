use std::io::{Read, stdin, Write};
use std::io::Error;
use std::net::TcpStream;

mod status;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();
    let mut password = String::new();

    let mut stream;
    let mut address = String::new();
    let mut port: u16 = 21;

    let mut data_stream: TcpStream;
    let mut data_stream_port: u16 = 21;

    (stream, address, port) = connect()?;

    (username, password) = login(&mut stream)?;

    send_command(&mut stream, "TYPE I")?;
    read_response(&mut stream)?;

    send_command(&mut stream, "PASV")?;
    let mut response = read_response(&mut stream)?;
    data_stream_port = parse_pasv_response(&response)?;

    data_stream = TcpStream::connect((address.to_owned(), data_stream_port))?;

    send_command(&mut stream, "LIST")?;
    let mut file_listing = String::new();
    data_stream.take(1024).read_to_string(&mut file_listing)?;

    println!("{}", file_listing);


    // PASS
    // send_command(&mut stream, "PASV")?;
    // read_response(&mut stream)?;
    //
    // send_command(&mut stream, "LIST")?;
    // read_response(&mut stream)?;

    println!("Done");

    Ok(())
}

fn parse_pasv_response(response: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let start = response.find('(').ok_or("Invalid PASV response")? + 1;
    let end = response.find(')').ok_or("Invalid PASV response")?;

    let parts: Vec<&str> = response[start..end].split(',').collect();

    let port = (parts[4].parse::<u16>()? >> 8) + parts[5].parse::<u16>()?;

    Ok(port)
}

fn console_readline() -> String {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("TODO: panic message");
    let res = buffer.trim_end().to_string();
    res
}


fn login(stream: &mut TcpStream) -> Result<(String, String), Error> {
    println!("Username:");
    let username = console_readline();
    println!();

    send_command(stream, format!("USER {}\r\n", username).as_str())?;
    read_response(stream)?;

    println!("Password:");
    let password = console_readline();
    println!();

    send_command(stream, format!("PASS {}\r\n", password).as_str())?;
    read_response(stream)?;

    Ok((username, password))
}

fn connect() -> Result<(TcpStream, String, u16), Error> {
    println!("address:");
    let mut address = console_readline();
    println!();

    println!("Port:");
    let mut port = console_readline().parse::<u16>().unwrap();
    println!();


    let stream = TcpStream::connect((address.to_owned(), port)).expect("TCP Stream hat sich eingeschissen");
    Ok((stream.try_clone()?, address, port))
}


fn send_command(stream: &mut TcpStream, command: &str) -> Result<(), Error> {
    let command = format!("{}\r\n", command);
    stream.write_all(command.as_bytes())?;
    Ok(())
}

fn read_response(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let res = String::from_utf8_lossy(&buffer).to_string();

    println!("{}", res);
    println!();
    Ok(res)
}