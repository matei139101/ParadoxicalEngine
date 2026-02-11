use configparser::ini::Ini;
use once_cell::sync::Lazy;

pub struct FileHandler {
    config: Ini
}

impl FileHandler {
    pub fn new() -> FileHandler {
        let mut config = Ini::new();
        let _ = config.load("/home/matei/Projects/ParadoxicalEngine/target/debug/config.ini");

        FileHandler { 
            config
        }
    }

    pub fn get_config(&self) -> &Ini {
        &self.config
    }
}

pub static FILE_HANDLER: Lazy<FileHandler> = Lazy::new(FileHandler::new);
