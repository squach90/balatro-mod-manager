pub enum ModLoader {
    Steamodded,
    Balamod,
}
pub struct CurrentModLoader(ModLoader);

pub struct ModCollection {
    pub name: String,
    pub path: String,
}

pub struct ModsPath {
    pub mods_collection: Option<Vec<ModCollection>>,
    pub path: String,
}

impl CurrentModLoader {
    pub fn new(current_modloader: ModLoader) -> Self {
        Self(current_modloader)
    }
    pub fn set(&mut self, modloader: ModLoader) {
        self.0 = modloader;
    }
    pub fn get(&self) -> &ModLoader {
        &self.0
    }

    pub fn initialize() {
        let steamodded_path = dirs::config_dir()
            .unwrap()
            .join("Balatro")
            .join("steamodded-mods");
        let balamod_path = dirs::config_dir()
            .unwrap()
            .join("Balatro")
            .join("balamod-mods");
        if !steamodded_path.exists() {
            std::fs::create_dir_all(&steamodded_path).unwrap();
        }
        if !balamod_path.exists() {
            std::fs::create_dir_all(&balamod_path).unwrap();
        }
    }

    pub fn get_path(&self) -> ModsPath {
        match self.0 {
            ModLoader::Steamodded => {
                let path = dirs::config_dir()
                    .unwrap()
                    .join("Balatro")
                    .join("steamodded-mods");

                ModsPath {
                    mods_collection: None,
                    path: path.to_string_lossy().to_string(),
                }
            }
            ModLoader::Balamod => {
                let path = dirs::config_dir()
                    .unwrap()
                    .join("Balatro")
                    .join("balamod-mods");

                ModsPath {
                    mods_collection: None,
                    path: path.to_string_lossy().to_string(),
                }
            }
        }
    }
}
