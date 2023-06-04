use glam::Vec3;
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
    pub info_screen: InfoScreenOptions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevOptions {
    pub camera: CameraOptions,
    pub player: PlayerOptions,
    pub light: LightOptions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGraphicsOptions {
    pub default_resolution: [u32; 2],
    pub max_fps: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CameraOptions {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LightOptions {
    pub light_color: [f32; 3],
    pub ambient_color: [f32; 3],
    pub linear_attenuation: f32,
    pub quadratic_attenuation: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoScreenOptions {
    pub font_name: String,
    pub font_size: f32,
    pub position: [f32; 2]
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

pub trait AsVector<T> {
    fn as_vector(&self) -> T;
}
impl AsVector<Vec3> for [f32; 3] {
    fn as_vector(&self) -> Vec3 {
        Vec3::new(self[0], self[1], self[2])
    }
}

pub trait AsTuple<T> {
    fn as_tuple(&self) -> T;
}
impl AsTuple<(f32, f32)> for [f32; 2] {
    fn as_tuple(&self) -> (f32, f32) {
        (self[0], self[1])
    }
}