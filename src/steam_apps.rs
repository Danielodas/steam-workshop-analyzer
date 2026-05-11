use reqwest;
use std::fs;
use std::path::Path;

pub struct SteamApp {
    app_id: String,
}

impl SteamApp {
    pub fn new(app_id: String) -> SteamApp {
        SteamApp { app_id }
    }

    pub async fn id_to_name(&self) -> String {
        let url = format!("https://store.steampowered.com/api/appdetails?appids={}", self.app_id);
        
        let response = reqwest::get(&url).await;

        match response {
            Ok(resp) => {
                let json: serde_json::Value = resp.json().await.unwrap();

                let name: &str = json[&self.app_id]["data"]["name"]
                    .as_str()
                    .unwrap_or("Unknown");

                name.to_string()
            }
            Err(_) => "Fetch error.".to_string(),
        }
    }

    pub async fn get_workshop_storage(os: String) -> f64 {
        let workshop_path: &Path;
        let mut total_size: f64 = 0.0;
        let formatted_path: String; // for Linux

        if os == "windows" {
            workshop_path = Path::new("C:/Program Files (x86)/Steam/steamapps/workshop/content");
        }
        else if os == "linux" {
            let home = std::env::var("HOME").expect("Home path not found");
            let steam_path = ".steam/steam/steamapps/workshop/content/";
            formatted_path = format!("{}/{}", home, steam_path);
            workshop_path = Path::new(&formatted_path);
        }
        else {
            println!("Operating system not supported...");
            return 0.0;
        }

        if !workshop_path.exists() {
            print!("Directory not found.");
            return 0.0;
        }

        println!("Analyzing {}...", workshop_path.display());
        println!("=============================");

        for entry in fs::read_dir(workshop_path).unwrap() {
            let entry = entry.unwrap();
            let size= get_directory_size(&entry.path());
            let size_mb = size as f64 / 1_048_576.0;

            let app_id = entry.file_name().to_string_lossy().to_string();
            let app = SteamApp::new(app_id);
            let name = app.id_to_name().await;
            
            total_size += size_mb;
            println!("{} - {:.2} mb", name, size_mb)
        }

        println!("=============================");
        print!("Total size: {:.2} mb", total_size);
        total_size
    }

}

fn get_directory_size(path: &Path) -> u64 {
    let mut total: u64 = 0;

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        if metadata.is_file() {
            total += metadata.len();
        }
        else if metadata.is_dir() {
            total += get_directory_size(&entry.path());
        }
    }
        
    total
}