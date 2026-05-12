mod steam_apps;

use std::io::{self, Write};
use std::env::args;
use std::env::consts::OS;
use steam_apps::SteamApp;

fn ascii_art() {
    println!("
    -++++++++++++++++-. ++++.    .-++-.    .++++ .-++++++++++++++++-                  
    +################+. ####.    .####.    .#### .+################+                  
    +###+               ####.    .####.    .#### .+###-        +###+                  
    +################+. ####.    .####.    .#### .+###-        +###+                  
    +################+. ####.    .####.    .#### .+###-........+###+                  
                 -###+. ####.    .####.    .#### .+################+                  
    +################+. ######################## .+###+--------####+                  
    +################+. ######################## .+###-        +###+
    ");
}

fn help() {
    ascii_art();
    println!();

    println!("Steam Workshop Analyzer, also known as SWA, is an open-source CLI tool");
    println!("meant to make it easier to analyze Steam workshop items, such as app IDs or");
    println!("the disk space taken by installed mods");
    println!("GitHub repository: https://github.com/Danielodas/steam-workshop-analyzer");

    println!();

    println!("Available arguments:");
    println!();
    
    println!("--help                Explains what SWA is.");
    println!("--disk                Analyzes the disk space your Steam workshop mods take in your system.");
    println!("--appid [appid]       Given a Steam app ID, returns its name.");
}

async fn check_args() -> bool {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        return false;
    }

    match args[1].as_str() {
        "--help" => {
            help();
            return true;
        },
        "--disk" => {
            SteamApp::get_workshop_storage(OS.to_string()).await;
            return true;
        },
        "--appid" => {
            let app_id: &String = &args[2];

            match app_id.parse::<String>() {
                Ok(app_id) => {
                    get_steamapp_by_id(app_id).await;
                    return true;
                },
                Err(_) => {
                    println!("invalid argument");
                    return true;
                }
            }
        },
        _ => println!("no matching argument"),
    }

    return true
}

fn menu() {
    println!("====================================");
    println!("0. Help");
    println!("1. Get Steam app name from id");
    println!("2. Analyze Steam workshop size");
    println!("3. Exit");
    println!("====================================");
}

async fn get_steamapp() {
    println!("Please, enter the app id:");
    let mut app_id = String::new();
    io::stdin().read_line(&mut app_id).unwrap();
    app_id = app_id.trim().to_string();

    get_steamapp_by_id(app_id).await;
}

async fn get_steamapp_by_id(app_id: String) {
    let steam_app: SteamApp = SteamApp::new(app_id.clone());
    let app_name = steam_app.id_to_name().await;

    print!("ID: {} - Name: {}", app_id.trim(), app_name);
    io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {

    let exit = check_args().await;
    let mut option = String::new();
    let mut wait = String::new();

    if exit {
        return;
    }

    loop {
        ascii_art();
        menu();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "0" => {
                help();
                io::stdin().read_line(&mut wait).unwrap();
            }
            "1" => {
                get_steamapp().await;
                io::stdin().read_line(&mut wait).unwrap();
            }
            "2" => {
                println!();
                SteamApp::get_workshop_storage(OS.to_string()).await;
                break;
            }
            "3" => {
                break;
            }
            _ => println!("Option out of scope."),
        }

        option = String::new();

    }

    println!("\n\nPress enter to exit...");
    io::stdin().read_line(&mut wait).unwrap();
}
