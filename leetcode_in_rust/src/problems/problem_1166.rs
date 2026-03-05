use std::collections::HashMap;

struct FileSystem {
    kv_store: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl FileSystem {

    fn new() -> Self {
        Self { kv_store: HashMap::new() }
    }

    fn create_path(&mut self, path: String, value: i32) -> bool {
        if self.kv_store.contains_key(&path) {
            return false;
        }
        let idx = path.rfind("/").unwrap();
        let prefix = &path[..idx];
        if !prefix.is_empty() && !self.kv_store.contains_key(prefix) {
            return false;
        }
        self.kv_store.insert(path, value);
        true
    }

    fn get(&self, path: String) -> i32 {
        match self.kv_store.get(&path) {
            Some(v) => { *v }
            None => {-1}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let mut fs = FileSystem::new();
        // [[],["/a",1],["/a"]]
        let create = fs.create_path(String::from("/a"), 1);
        assert_eq!(true, create);
        let get = fs.get(String::from("/a"));
        assert_eq!(get, 1);
    }

    #[test]
    fn unit_test2() {
        let mut fs = FileSystem::new();
        // [[],["/leet",1],["/leet/code",2],["/leet/code"],["/c/d",1],["/c"]]
        let create = fs.create_path(String::from("/leet"), 1);
        assert_eq!(true, create);
        let create2 = fs.create_path(String::from("/leet/code"), 2);
        assert_eq!(true, create2);
        let get = fs.get(String::from("/leet/code"));
        assert_eq!(get, 2);

        let create3 = fs.create_path(String::from("/c/d"), 1);
        assert_eq!(false, create3);
        let get2 = fs.get(String::from("/c"));
        assert_eq!(get2, -1);
    }
}
