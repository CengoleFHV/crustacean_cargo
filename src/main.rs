mod ftp_client;

use std::error::Error;

use ftp_client::FtpClient;

fn main() -> Result<(), Box<dyn Error>> {
    let address: String = "127.0.0.1".to_string();
    let port: u16 = 21;

    let username = "user";
    let password = "user";

    let mut ftp_client: FtpClient = FtpClient::connect(address, port)?;

    ftp_client.login(username, password)?;

    ftp_client.list_files("/files")?;

    ftp_client.get("/files/jazz.jpg")?;

    ftp_client.mget(&vec![
        "/files/founding_fathers.txt",
        "/files/dickbutt.txt",
        "/files/readme.txt",
    ])?;

    ftp_client.quit()?;

    Ok(())
}
