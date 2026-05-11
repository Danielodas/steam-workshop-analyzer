mod steam_apps;

use std::io;
use std::env::consts::OS;
use steam_apps::SteamApp;

#[tokio::main]
async fn main() {

    println!("Select what you want to do:");
    println!("====================================");
    println!("1. Get Steam app name from id");
    println!("2. Analyze Steam workshop size");
    println!("====================================");
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();

    match option.trim() {
        "1" => {
            println!("Please, enter the app id:");
            let mut app_id = String::new();
            io::stdin().read_line(&mut app_id).unwrap();
            app_id = app_id.trim().to_string();

            let steam_app: SteamApp = SteamApp::new(app_id.clone());
            let app_name = steam_app.id_to_name().await;

            print!("ID: {} - Name: {}", app_id.trim(), app_name);
        }
        "2" => {
            println!();
            SteamApp::get_workshop_storage(OS.to_string()).await;
        }
        _ => println!("Option out of scope."),
    }

    println!("\n\nPress enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
