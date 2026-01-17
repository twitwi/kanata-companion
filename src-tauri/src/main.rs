// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::BufRead;
use tauri::{AppHandle, Emitter};

// a set of constants / config
const HOST: &str = "localhost";
const PORT: u16 = 42041;

  // create a new thread that connects
  // to the tcp server at HOST:PORT, reconnecting if the connection is lost
  // it reads json messages (linewise)

#[tauri::command]
fn ping_pong(app: AppHandle) {
    app.emit("ping-pong", "init1").unwrap();
}
#[tauri::command]
fn start_kanata_listener(app: AppHandle) {
    app.emit("kanata-message", "init1").unwrap();
    
    std::thread::spawn(move || {
    app.emit("kanata-message", "init2").unwrap();
        loop {
            match std::net::TcpStream::connect((HOST, PORT)) {
                Ok(stream) => {
                    let reader = std::io::BufReader::new(stream.try_clone().unwrap());
                    for line in reader.lines() {
                    //loop {
                    //  let line = reader.read_line();
                        match line {
                            Ok(json_msg) => {
                                // process the json message
                                app.emit("kanata-message", "before").unwrap();
                                println!("Received: {}", json_msg);
                                app.emit("kanata-message", json_msg).unwrap();
                            }
                            Err(e) => {
                                eprintln!("Error reading from stream: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to connect: {}", e);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        }
    });
}




fn main() {

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_kanata_listener, ping_pong])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  //start_kanata_listenera();
  //app_lib::run();
}
