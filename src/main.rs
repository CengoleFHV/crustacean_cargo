mod ftp_client;

use std::{
    error::Error,
    io::{stdin, Read},
};

use ftp_client::FtpClient;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Ip address:");
    // let address: String = "127.0.0.1".to_string();

    //set let address with console input
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
                ftp_client.ascii()?;
            }
            "binary" => {
                ftp_client.binary()?;
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
