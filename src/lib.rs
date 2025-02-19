pub mod driver_manager;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Driver {
    pub name: String,
    pub version: String,
    pub download_url: String,
}

pub struct DriverManager {
    client: Client,
}

impl DriverManager {
    pub fn new() -> Self {
        let client = Client::new();
        DriverManager { client }
    }

    pub async fn fetch_drivers(&self) -> Result<Vec<Driver>, reqwest::Error> {
        let response: DriverResponse = self.client.get("https://api.example.com/drivers")
            .send()
            .await?
            .json()
            .await?;
        Ok(response.drivers)
    }

    pub fn download_driver(&self, driver: &Driver) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(&driver.download_url).send()?;
        let path = Path::new(&driver.name).with_extension("exe");
        let mut file = fs::File::create(&path)?;
        let content = response.bytes()?;
        file.write_all(&content)?;
        Ok(())
    }
}