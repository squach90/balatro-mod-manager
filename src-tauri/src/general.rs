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

    pub fn get_path(&self) -> ModsPath {
        match self.0 {
            ModLoader::Steamodded => {
                let path = std::env::var("STEAMODDED_PATH").unwrap_or_else(|_| "".to_string()); // TODO:implement
                ModsPath {
                    mods_collection: None,
                    path,
                }
            }
            ModLoader::Balamod => {
                let path = std::env::var("BALAMOD_PATH").unwrap_or_else(|_| "".to_string()); // TODO:implement

                ModsPath {
                    mods_collection: None,
                    path,
                }
            }
        }
    }
}
