use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use sfml::graphics::{RenderTexture, Texture};
use sfml::SfBox;

pub struct ResourceManager {
    textures: HashMap<String, Option<SfBox<Texture>>>,
}

impl ResourceManager {
    pub fn new(path: &str) -> Self {
        let paths = fs::read_dir(path).unwrap();

        let mut textures = HashMap::new();

        for entry in paths {
            let pathBuf = entry.unwrap().path();
            let path = pathBuf.display().to_string();
            let file_name = pathBuf.file_name().unwrap().to_str().unwrap().to_string();

            println!("Loaded {}: {}", file_name.as_str(), path);
            textures.insert(file_name, Texture::from_file(path.as_str()));
        }

        return ResourceManager { textures };
    }
}

