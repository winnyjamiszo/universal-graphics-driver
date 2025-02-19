use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Driver {
    name: String,
    version: String,
    download_url: String,
}

#[derive(Serialize, Deserialize)]
struct DriverResponse {
    drivers: Vec<Driver>,
}

struct DriverManager {
    client: Client,
}

impl DriverManager {
    fn new() -> Self {
        let client = Client::new();
        DriverManager { client }
    }

    async fn fetch_drivers(&self) -> Result<Vec<Driver>, reqwest::Error> {
        let response: DriverResponse = self.client.get("https://api.example.com/drivers")
            .send()
            .await?
            .json()
            .await?;
        Ok(response.drivers)
    }

    fn download_driver(&self, driver: &Driver) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(&driver.download_url).send()?;
        let path = Path::new(&driver.name).with_extension("exe");
        let mut file = fs::File::create(&path)?;
        let content = response.bytes()?;
        file.write_all(&content)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let driver_manager = DriverManager::new();
    match driver_manager.fetch_drivers().await {
        Ok(drivers) => {
            for driver in drivers {
                if let Err(e) = driver_manager.download_driver(&driver) {
                    eprintln!("Failed to download {}: {}", driver.name, e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching drivers: {}", e);
        }
    }
}