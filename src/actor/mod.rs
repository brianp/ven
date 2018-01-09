use std::net::UdpSocket;
use std::path::PathBuf;
use std::process::{Command, exit};
use notify::op;
use notify;

pub fn listen_and_mimic(socket: &UdpSocket) {
    println!("listening started, ready to accept");

    let mut buf = [0; 64000];
    println!("Reading data");

    loop {
        let result = socket.recv_from(&mut buf);

        drop(socket);

        match result {
            Ok((amt, src)) => {
                let data = Vec::from(&buf[..amt]);
                let string = String::from_utf8_lossy(&data);
                println!("received data from {}: {}", src, &string);
                execute(string.into_owned());
            },
            Err(err) => panic!("Read error: {}", err)
        }
     }
}

fn execute(data: String) {
    let vec: Vec<&str> = data.split("|").collect();
    let file: PathBuf = PathBuf::from(vec[1]);
    let file_str: &str = file.to_str().expect("Couldn't parse file path");

    match vec[0] {
        "CREATE" => {
            Command::new("sed")
                    .args(&["-n", "", "-i", file_str])
                    .spawn()
                    .expect("sed command failed to start");
        },
        &_ => {
            Command::new("touch")
                    .args(&["-r", file_str, file_str])
                    .spawn()
                    .expect("touch command failed to start");
        }
    }

}
