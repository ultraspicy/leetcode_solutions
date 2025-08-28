use std::{collections::HashMap, collections::HashSet};

#[derive(Default, Debug)]
struct FileSystem {
    directories: HashMap<String, HashSet<String>>,
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
        let file_option = self.files.get(&path);
        if file_option.is_some() {
            return vec![path.rsplit("/").next().unwrap().chars().collect::<String>()]
        }
        let key = if path == "/" { String::new()} else {path};
        let ret = self.directories.get(&key).unwrap_or(&HashSet::new()).clone();
        let mut ordered_list = ret.into_iter().collect::<Vec<_>>();
        ordered_list.sort();
        ordered_list
    }

    fn mkdir(&mut self, path: String) {
        // store full path & recursively create directory
        _ = self._mkdir(path);
    }

    fn _mkdir(&mut self, path: String) -> String {
        // store full path & recursively create directory
        let sub_directories = path.split("/").collect::<Vec<_>>();
        let mut cur_full_path: Vec<char> = vec![];
        for i in 1..sub_directories.len() {
            let mut cur_relative_path = sub_directories[i].chars().collect::<Vec<char>>();
            self.directories
                .entry(cur_full_path.iter().collect::<String>())
                .or_insert_with(HashSet::new)
                .insert(String::from(sub_directories[i]));
            cur_full_path.push('/');

            cur_full_path.append(&mut cur_relative_path);

        }
        cur_full_path.iter().collect::<String>()
    }

    fn add_content_to_file(&mut self, file_path: String, content: String) {
        self.mkdir(file_path.clone());
        //self.files.insert(file_path, content);
        let existing = if self.files.get(&file_path).is_some() {
            self.files.remove(&file_path).unwrap()
        } else {
            String::from("")
        };
        let new_content = existing.chars().chain(content.chars()).collect::<String>();
        self.files.insert(file_path, new_content);
    }

    fn read_content_from_file(&self, file_path: String) -> String {
        self.files.get(&file_path).unwrap_or_else(|| panic!("the file path doesn't exist")).clone()
    }

    fn print(&self) {
        println!("========================");
        println!("directories = {:?}", self.directories);
        println!("file = {:?}", self.files);
    }
}

#[cfg(test)]
mod test {
    use crate::problems::problem_588::FileSystem;

    #[test]
    fn unit_test() {
        let mut fs = FileSystem::new();

        let rst = fs.ls(String::from("/"));
        assert_eq!(Vec::<String>::new(), rst);

        fs.mkdir(String::from("/a/b/c"));
        fs.print();
        fs.add_content_to_file(String::from("/a/b/c/d"), String::from("hello"));
        fs.print();
        let rst = fs.ls(String::from("/"));
        assert_eq!(vec!["a"], rst);

        let rst = fs.read_content_from_file(String::from("/a/b/c/d"));
        assert_eq!(String::from("hello"), rst);
    }
}
