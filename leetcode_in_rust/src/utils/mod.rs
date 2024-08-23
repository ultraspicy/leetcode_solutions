pub use self::trie::Trie;

// Declare the trie module, which corresponds to the trie.rs file
pub mod trie;

// Utility function for printing vectors
pub fn print_vec<T: std::fmt::Display>(vec: &Vec<T>) {
    for e in vec {
        println!("{} \t", e);
    }
}

// You can add more utility functions here
pub fn pow(base: i64, exp: i64) -> i64 {
    let m: i64 = 1_000_000_000 + 7;
    let mut ret = 1;
    for _i in 0..exp {
        ret = (ret * base) % m;
    }
    ret
}