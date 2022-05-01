use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Loader {
    path : PathBuf,
    textures : HashMap<String, Texture2D>,

}

impl Loader {
    pub fn new(path : &str) -> Loader {
        // let p = std::env::current_dir().unwrap();
        // println!("cwd : {}", p.display());
        Loader {
            path : PathBuf::from(path),
            textures : HashMap::new(),
        }
    }
    pub fn get_path(&self) -> &std::path::Path {
        self.path.as_path()
    }

    pub async fn load_resources(&mut self) {
        self.load_textures().await;
        // load_sounds() ...
    }

    pub fn get_texture_2d(&self, key : &str) -> Texture2D {
        // let texture = self.textures.remove(key).unwrap(); // remove texture from hashmap, but needs &mut self into signature
        let texture = self.textures.get(key).unwrap().clone(); // keep texture into the hashmap
        texture
    }

    pub fn get_ref_texture(&self, key : &str) -> &Texture2D {
        self.textures.get(key).unwrap()
    }

    pub fn print_textures(&self) {
        println!("textures : {:?}", self.textures);
    }
    
    fn get_list_of_images(&mut self) -> HashMap<String, PathBuf> {
        let mut paths : HashMap<String, PathBuf> = HashMap::new();
        for entry in self.path.read_dir().expect("failed to read dir") {
            if let Ok(entry) = entry {
                let extension = entry.path();
                let extension = extension.extension().unwrap().to_str();
                if let Some("png") = extension {
                    let name = entry.path().clone();
                    let name = name.as_path().file_stem().unwrap().to_str().unwrap().to_owned();
                    paths.insert(name, entry.path());
                    // println!("read entry : {:?}", paths);
                }
            }
        }
        paths
    }

    pub async fn load_textures(&mut self){
        // load all textures from the assets directory
        let paths = self.get_list_of_images();
        for (name, file) in paths.iter() {
            let tex2d = load_texture(file.to_str().unwrap()).await.unwrap();
            self.textures.insert(name.to_owned(), tex2d);
        }
        //self.print_textures();
    }
}