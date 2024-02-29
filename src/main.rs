mod ftp_client;

use ftp_client::FtpClient;
use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Ip address:");
    let address = console_readline();

    println!("Port:");
    let port: u16 = console_readline().parse::<u16>().unwrap();

    println!("Username:");
    let username = console_readline();

    println!("Password:");
    let password = console_readline();

    let mut ftp_client: FtpClient = FtpClient::connect(address, port)?;
    ftp_client.login(&username, &password)?;

    loop {
        println!("Enter command:");
        let command = console_readline();

        match command.as_str() {
            "list" => {
                println!("Enter path:");
                ftp_client.list_files(&console_readline())?;
            }
            "get" => {
                println!("Enter file path:");
                ftp_client.get(&console_readline())?;
            }
            "mget" => {
                println!("Enter file paths separated by comma:");
                let paths = console_readline();
                let paths: Vec<&str> = paths.split(",").collect();
                ftp_client.mget(&paths)?;
            }
            "ascii" => {
                ftp_client.set_ascii_mode()?;
            }
            "binary" => {
                ftp_client.set_binary_mode()?;
            }
            "help" => {
                println!("Commands: list, get, mget, ascii, binary, help, quit");
            }
            "quit" => break,
            _ => println!("Invalid command"),
        }

        if command == "quit" {
            ftp_client.quit()?;
            break;
        }
    }
    Ok(())
}

fn console_readline() -> String {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("TODO: panic message");
    let res = buffer.trim_end().to_string();
    res
}
