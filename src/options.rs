use serde::{Deserialize, de::DeserializeOwned, Serialize};
use std::fs;

#[derive(Debug)]
pub struct Options {
    pub user: UserOptions,
    pub dev: DevOptions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserOptions {
    pub graphics: UserGraphicsOptions,
    pub input: InputOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevOptions {
    pub graphics: DevGraphicsOptions,
    pub player: PlayerOptions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGraphicsOptions {
    pub default_resolution: [u32; 2],
    pub max_fps: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevGraphicsOptions {
    pub fov: f32,
    pub near_plane: f32,
    pub far_plane: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputOptions {
    pub mouse_sensitivity: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerOptions {
    pub walking_speed: f32
}

impl Options {
    pub fn load() -> Options {
        Options {
            user: load_from_file("Resources/options.json"),
            dev: load_from_file("Resources/dev_options.json")
        }
    }
}

fn load_from_file<T: DeserializeOwned>(path: &str) -> T {
    let json = fs::read_to_string(path).unwrap();

    return serde_json::from_str(json.as_str())
        .expect(format!("Failed to parse file: {}", path).as_str());
}