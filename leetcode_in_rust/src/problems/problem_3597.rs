use super::Solution;


#[derive(Default, Debug)]
pub struct Trie {
    ending: bool,
    children: [Option<Box<Trie>>; 26],
} 

impl Trie {
    pub fn insert(&mut self, s: &str) {
        let mut cur = self;
        for c in s.chars() {
            cur = cur.children[c as usize - 'a' as usize]
                .get_or_insert_with(|| Box::new(Trie::default()));
        }
        cur.ending = true;
    }

    pub fn find(&self, s: &str) -> bool {
        let mut current = self;
        for c in s.chars() {
            let i = c as usize - 'a' as usize;
            if let Some(ref new_cur) = current.children[i] {
                // box deref coercion
                current = new_cur;
            } else {
                return false;
            }
        }
        current.ending
    }
}

impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
        let mut root = Trie::default();
        let mut current = &mut root; 
        let mut chars: Vec<char> = Vec::new();
        let mut ret = vec![];

        for c in s.chars() {
            chars.push(c);
            let child_index = c as usize - 'a' as usize;
            current = current.children[child_index]
                .get_or_insert_with(||Box::new(Trie::default()))
                .as_mut();
            if !current.ending {
                current.ending = true;
                current = &mut root;
                
                ret.push(chars.iter().collect::<String>());
                chars = Vec::new();
            }
        }
        ret
    }
}

// following are two slow implementation. 
// nth(i) or chars.iter().collect::<String>() incurs a lot of duplicated operation
// for example , "a", then "ab", "abc" need to be created one after another
// a smart solution would be use a moving pointer on a trie 
// and we don't need to build a substring everytime to check if the string 
// preexists. We just create string when we do need to add it in return

// impl Solution {
//     pub fn partition_string(s: String) -> Vec<String> {
//         let mut set: HashSet<String> = HashSet::new();
//         let mut ret = Vec::new();

//         let mut chars= Vec::new();

//         (0..s.len()).collect::<Vec<_>>()
//             .into_iter()
//             .for_each(|i| {
//                 chars.push(s.chars().nth(i).unwrap());
//                 if set.insert(String::from_iter(chars.iter())) {
//                     ret.push(String::from_iter(chars.iter()));
//                     chars = Vec::new();
//                 }
//             });
    

//         ret
//     }
// }

// pub fn partition_string(s: String) -> Vec<String> {
//         let mut root = trie::Trie::default();
//         let mut chars: Vec<char> = Vec::new();
//         let mut ret = vec![];

//         for c in s.chars() {
//             chars.push(c);
//             let s = chars.iter().collect::<String>();
//             if !root.find(&s) {
//                 root.insert(&s);
//                 ret.push(s);
//                 chars = Vec::new();
//             }
//         }
//         ret
//     }