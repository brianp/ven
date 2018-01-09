use notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, op};
use notify::DebouncedEvent::*;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::net::UdpSocket;
use std::net;
use std::env;
use std::path::PathBuf;
use std::process::exit;

pub fn listen_and_broadcast(socket: &UdpSocket, bind_addr: &net::SocketAddrV4) {
    println!("It's stream time");

    if let Err(e) = watch(socket, bind_addr) {
        println!("error: {:?}", e)
    }
}

fn watch(socket: &UdpSocket, bind_addr: &net::SocketAddrV4) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));
    let path: PathBuf = env::current_dir().expect("Current directory is invalid for watching");

    try!(watcher.watch(path.as_path(), RecursiveMode::Recursive));

    loop {
        match rx.recv() {
            Ok(event) => {
                let data = data(event);
                println!("sending data to {}: {:?}", &bind_addr, &data);
                socket.send_to(data.as_bytes(), bind_addr).expect("couldn't send message");
            },
            Err(e) => println!("watch error: {:?}", e)
        }
    }
}

fn data(event: notify::DebouncedEvent) -> String {
    match event {
        NoticeWrite(x) => data_format(op::WRITE, x),
        NoticeRemove(x) => data_format(op::REMOVE, x),
        Create(x) => data_format(op::CREATE, x),
        Write(x) => data_format(op::WRITE, x),
        Chmod(x) => data_format(op::CHMOD, x),
        Remove(x) => data_format(op::REMOVE, x),
        Rename(_, x) => data_format(op::RENAME, x),
        Rescan => exit(1),
        Error(_, _) => exit(1)
   }
}

fn data_format(event: notify::Op, path: PathBuf) -> String {
    format!("{:?}|{}", event, path.display())
}
