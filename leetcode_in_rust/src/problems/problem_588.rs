use std::{collections::HashMap, default};

#[derive(Default, Debug)]
struct FileSystem {
    directories: HashMap<String, Vec<String>>,
    files: HashMap<String, String>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSystem {

    fn new() -> Self {
        FileSystem::default()
    }

    fn ls(&self, path: String) -> Vec<String> {
        let subpath = path.split('/').collect::<Vec<&str>>();
        

        []
    }

    fn mkdir(&self, path: String) {

    }

    fn add_content_to_file(&self, file_path: String, content: String) {

    }

    fn read_content_from_file(&self, file_path: String) -> String {

    }
}
