use std::error::Error;
use std::fs::File;
// Add missing import
use std::{
    io::{Read, Write},
    net::TcpStream,
};

mod status;

pub struct FtpClient {
    control_stream: TcpStream,
}

impl FtpClient {
    pub fn connect(address: String, port: u16) -> Result<FtpClient, Box<dyn Error>> {
        let stream: TcpStream =
            TcpStream::connect((address, port)).expect("Connecting TcpStream failed");

        let mut ftp_client: FtpClient = FtpClient {
            control_stream: stream,
        };

        let _ = ftp_client.read_response(None)?;

        Ok(ftp_client)
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        println!(
            "Logging in with\r\nusername: {} and\r\npassword: {}\r\n",
            username, password
        );
        let _ = Self::send_command(&mut self.control_stream, format!("USER {}\r\n", username))?;
        let res: String = self.read_response(None)?;

        let code = res[0..3].parse::<u32>()?;

        if code == status::NEED_PASSWORD {
            let _ = Self::send_command(&mut self.control_stream, format!("PASS {}\r\n", password))?;
            self.wait_for_code(vec![status::LOGGED_IN])?;
        }

        println!("Login Successful\r\n");

        Ok(())
    }

    pub fn list_files(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let _ = Self::send_command(&mut self.control_stream, "PASV\r\n".to_string())?;

        let res = self.read_response(None)?;

        if res.contains(&status::PASSIVE_MODE.to_string()) {
            let data_address: String;
            let data_port: u16;

            (data_address, data_port) = Self::parse_pasv_response(&res);

            let mut data_stream: TcpStream = TcpStream::connect((data_address, data_port))?;

            let _ = Self::send_command(&mut self.control_stream, format!("LIST {}\r\n", path))?;

            self.wait_for_code(vec![status::ABOUT_TO_SEND])?;

            let mut file_listing: String = String::new();
            let _ = data_stream.read_to_string(&mut file_listing);

            println!("Files in the current directory:\n{}", file_listing);
        }

        //loop and read response and wait until the response contains status::CLOSING_DATA_CONNECTION as a string

        self.wait_for_code(vec![status::CLOSING_DATA_CONNECTION])?;

        Ok(())
    }

    pub fn get(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let _ = Self::send_command(&mut self.control_stream, "PASV\r\n".to_string())?;

        let res = self.read_response(None)?;

        if res.contains(&status::PASSIVE_MODE.to_string()) {
            let data_address: String;
            let data_port: u16;

            (data_address, data_port) = Self::parse_pasv_response(&res);

            let mut data_stream: TcpStream = TcpStream::connect((data_address, data_port))?;

            let _ = Self::send_command(&mut self.control_stream, format!("RETR {}\r\n", path))?;

            self.wait_for_code(vec![status::ABOUT_TO_SEND])?;

            let mut buffer = vec![0; 262144];
            let bytes_read = data_stream.read(&mut buffer)?;
            let content = buffer[0..bytes_read].to_vec();

            let filename = path.split("/").last().unwrap();

            let mut file: File = File::create(filename)?;

            file.write(&content)?;

            println!("File '{}' retrieved successfully", filename);
        }

        self.wait_for_code(vec![status::CLOSING_DATA_CONNECTION])?;

        Ok(())
    }

    pub fn mget(&mut self, paths: &Vec<&str>) -> Result<(), Box<dyn Error>> {
        for path in paths.iter() {
            self.get(path)?;
        }

        Ok(())
    }

    //public function that can send the Command TYPE I and TYPE A to ask for Binary or ASCII mode
    pub fn set_binary_mode(&mut self) -> Result<(), Box<dyn Error>> {
        let _ = Self::send_command(&mut self.control_stream, "TYPE I\r\n".to_string())?;
        self.read_response(None)?;

        Ok(())
    }

    pub fn set_ascii_mode(&mut self) -> Result<(), Box<dyn Error>> {
        let _ = Self::send_command(&mut self.control_stream, "TYPE A\r\n".to_string())?;
        self.read_response(None)?;

        Ok(())
    }

    fn wait_for_code(&mut self, code: Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut condition_satisfied: bool = false;

        loop {
            if !condition_satisfied {
                let res = self.read_response(None)?;
                let lines = res.split("\r\n");

                for line in lines {
                    if !line.is_empty() {
                        let parts: Vec<&str> = line.split(" ").collect();

                        if let Ok(line_status_code) = parts[0].parse::<u32>() {
                            if code.contains(&line_status_code) {
                                condition_satisfied = true;
                                break;
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn parse_pasv_response(response: &str) -> (String, u16) {
        let start = response.find('(').ok_or("Invalid PASV response").unwrap() + 1;
        let end = response.find(')').ok_or("Invalid PASV response").unwrap();

        let parts: Vec<&str> = response[start..end].split(',').collect();

        let port = (parts[4].parse::<u16>().unwrap() * 256) + parts[5].parse::<u16>().unwrap();
        let address = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], parts[3]);

        (address, port)
    }

    fn send_command(stream: &mut TcpStream, command: String) -> Result<(), Box<dyn Error>> {
        stream
            .write_all(command.as_bytes())
            .expect("Writing to Stream failed");

        Ok(())
    }

    fn read_response(&mut self, stream: Option<&mut TcpStream>) -> Result<String, Box<dyn Error>> {
        let mut buffer = [0; 1024];

        if stream.is_none() {
            self.control_stream
                .read(&mut buffer)
                .expect("Stream Read Failed");
        } else {
            stream
                .unwrap()
                .read(&mut buffer)
                .expect("Stream Read Failed");
        }

        let res: String = String::from_utf8_lossy(&buffer).to_string();

        println!("{}", res);
        println!();
        Ok(res)
    }
}
