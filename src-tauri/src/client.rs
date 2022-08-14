use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::fs::File;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use tauri::Window;
use sha256::digest_bytes;

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    kind:String,
    name:String,
    hash:String,
    total_size:u64,
    file_size: u64
}

pub fn calculate_size(path:String)->u64{
    let mut calculated_size:u64 = 0;
    for entry in WalkDir::new(path) {
        let path = entry.as_ref().unwrap().path();
        
        // if path is folder
        if path.is_file() {
            let file = File::open(path.display().to_string()).unwrap();
            calculated_size += file.metadata().unwrap().len() as u64;
        };
    }
    calculated_size
}

pub fn start_sending(window:Window,path:String,address:String,total_size:u64) -> io::Result<()> {
    println!("path {}",path);

    let base_path = basename(&path); //leave only last part
println!("base_path {}",base_path);
    for entry in WalkDir::new(path) {
        let path = entry.as_ref().unwrap().path();
        
        // if path is folder
        if entry.as_ref().unwrap().path().is_dir() {
           send_folder(path.display().to_string(),&base_path,&address,total_size);
        }else{
            while !send_file(&window,path.display().to_string(),&base_path,&address,total_size){
                println!("sending failed, trying again");
            };
        }
    }

    
   
    Ok(())

}
fn send_folder(path:String,base_path:&str,address:&str,total_size:u64){
    let mut tcp = TcpStream::connect(address).unwrap();
    let header = Header{
        kind:"folder".to_string(),
        name: path.replace(&base_path.to_string(),"./"),
        file_size: 0,
        total_size: total_size,
        hash: "".to_string()
    };
    let header_json = serde_json::to_string(&header).unwrap();
    println!("{}", header_json);
    tcp.write_all(&header_json.as_bytes()).unwrap();
    
}

fn send_file(window:&Window,file_name:String,base_path:&str,address:&str,total_size:u64)->bool{
        println!("file_name {}",file_name);
        println!("base_path {}",base_path);

        let mut file = File::open(&file_name).unwrap();
        let file_hash = hash_file(file);
        println!("{}",file_hash);
        let mut file = File::open(&file_name).unwrap();
        let mut tcp = TcpStream::connect(address).unwrap();
        
        let mut send_buffer = [0; 4096];
        let mut recv_buffer = [0; 4096];
        let header = Header{
            kind:"file".to_string(),
            name: file_name.replace(&base_path.to_string(),"./"),
            file_size: file.metadata().unwrap().len() as u64,
            total_size:total_size,
            hash: file_hash
        };
        let header_json = serde_json::to_string(&header).unwrap();
        println!("{}", header_json);
        tcp.write_all(&header_json.as_bytes()).unwrap();
        let _rn = tcp.read(&mut recv_buffer).unwrap(); //"received header" message from server
        loop {
            
            let n = file.read(&mut send_buffer).unwrap();
            
            if n == 0 {
                println!("{}", "EOF");
                tcp.flush().unwrap();  
                println!("{}", "waiting for server");
                let _rn = tcp.read(&mut recv_buffer).unwrap(); // waiting the response from server if file was correctly sent
                
                let resp = std::str::from_utf8(&recv_buffer[.._rn]);
                let resp = resp.unwrap(); 
                println!("response is {}", if resp=="true"{"hashes match"}else{"hashes don't match"});     
                // reached end of file
                break resp=="true";
            }
            
            
           
                tcp.write_all(&send_buffer[..n]).unwrap();
            
        
           window.emit("progress",n);
            
        }    
       
      
       
}


fn basename<'a>(path: &'a str) -> String {
    let mut pieces = path.rsplitn(2, |c| c == '/' || c == '\\');
    let last:_ = match pieces.next() {
        Some(p) => <&str as Into<String>>::into(p),
        None => path.into(),
    };
    path.replacen(&last,"",1).replace(r"\\",r"\")
}


pub fn hash_file(mut file:File)->String{
    let mut buffer = [0; 4096];
    let mut result_hash = "".to_string();
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            println!("{}", "EOF Finished hashing");
            println!("hash is {}", result_hash);
            // reached end of file
            break;
        }
      let  hash =digest_bytes(&buffer[..n]);
      result_hash = digest_bytes(format!("{}{}",result_hash,hash).as_bytes());

    }
    result_hash
}
