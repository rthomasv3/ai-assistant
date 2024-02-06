use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Config {
    pub prefer_mmap: bool,
    pub context_size: usize,
    pub use_gpu: bool,
    pub gpu_layers: Option<usize>,
    pub model_folder: Option<String>,
    pub model_path: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let local_data_dir = tauri::api::path::local_data_dir().unwrap();
        let app_data_path = Path::new(&local_data_dir).join("ai-assistant");
        fs::create_dir_all(&app_data_path).expect("Unable to create app settings directory.");
        let app_settings_path = app_data_path.join("config.json");
        if app_settings_path.exists() {
            let data = fs::read_to_string(app_settings_path).unwrap();
            serde_json::from_str(&data).unwrap()
        } else {
            dotenv::dotenv().ok();
            let config = Config {
                model_folder: Some(String::from(app_data_path.to_str().unwrap())),
                model_path: Some(std::env::var("MODEL_PATH").expect("MODEL_PATH must be set")),
                ..Default::default()
            };
            let data = serde_json::to_string_pretty(&config).unwrap();
            fs::write(app_settings_path, data).unwrap();
            config
        }
    }

    pub fn update_model_path(&mut self, model_path: &str) {
        // update value
        self.model_path = Some(model_path.to_string());

        // write to config file
        let local_data_dir = tauri::api::path::local_data_dir().unwrap();
        let app_data_path = Path::new(&local_data_dir).join("ai-assistant");
        let app_settings_path = app_data_path.join("config.json");
        let data = serde_json::to_string_pretty(&self).unwrap();
        fs::write(app_settings_path, data).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefer_mmap: true,
            context_size: 2048,
            use_gpu: true,
            gpu_layers: None,
            model_folder: None,
            model_path: None,
        }
    }
}
