#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::Window;
mod client;
mod server;

#[tauri::command]
fn start_sending(window:Window,path:String,address:String,total_size:u64) -> String{
match client::start_sending(window,path,address,total_size){
    Ok(_)=>"Finished".to_string(),
    Err(e)=>"error".to_string(),
}
}
#[tauri::command]
fn calculate_size(path:String) -> u64{
    client::calculate_size(path)


}

#[tauri::command]
fn start_server(window:Window,folder_path:String,address:String){
  println!("{}",folder_path);
  std::thread::spawn(move ||{
    server::start_server(window,folder_path,address);
  });
}


fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![start_sending,calculate_size,start_server ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
