use crate::client;
use std::net::TcpListener;
use std::fs::{File,OpenOptions};
use std::io::{Read,Write};
use sha256::{digest_bytes,digest};
use serde::{Serialize, Deserialize};
use tauri::Window;
#[derive(Serialize, Deserialize, Debug)]
struct Header {
    kind:String,
    name:String,
    hash:String,
    total_size:u64,
    file_size: u64
}

pub fn start_server(window:Window,folder_path:String,adress:String,) {
    let listener = TcpListener::bind(adress).unwrap();
    window.emit("server_status",true);
    //append to File
    let mut total_received:u64=0;  
    let mut total_size:u64=0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();  
  
        stream.set_read_timeout(Some(std::time::Duration::from_millis(8000)));
        let mut received_file_size:u64=0;
    
        let mut stream = stream;       
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).unwrap();
        let header_json = std::str::from_utf8(&buf[..n]).unwrap();
        stream.write(b"received header").unwrap();
        println!("{}",header_json);
        println!("{}",folder_path);
        let header: Header = serde_json::from_str(header_json).unwrap();
        let path =  format!("{}/{}",&folder_path,&header.name); // where to save, &header.name is relative path from client
        if total_size != header.total_size {
            total_received=0;
            total_size = header.total_size;
        }
        if header.kind == "folder"{
            std::fs::create_dir_all(path).unwrap();
            
                
                continue;
            }

            File::create(&path).unwrap();
            let mut file = OpenOptions::new().read(true).append(true).open(&path).unwrap();
            loop {
            if received_file_size == header.file_size {
                break;
            }
            let n = match stream.read(&mut buf) {
                Ok(n) => n,
                Err(e) => 0
            };
            if n == 0 {
                    println!("server n==0")  ;        // reached end of file
               break
            }

            received_file_size += n as u64;
            total_received+=n as u64;
            let progress =total_received * 100/header.total_size;
        
            println!("{}", total_received);
            window.emit("progress",progress);
            
            file.write(&buf[..n]).unwrap();
           
        }

        
        let mut file = File::open(&path).unwrap();
        let hash = client::hash_file(file);
        println!("{}",hash == header.hash);
        if hash == header.hash {
            stream.write(b"true").unwrap();
            println!("{}","sent true");
        }else{
            stream.write(b"false").unwrap();
            println!("{}","sent false");
        }
    }
}


