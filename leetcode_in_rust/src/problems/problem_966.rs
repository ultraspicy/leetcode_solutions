use std::{collections::{HashMap, HashSet}, hash::Hash};

use super::Solution;

// use map.entry() to always keep the first one
// also, can use map.insert for unconditionally overwrite to keep last one
// the old version (commmented out code) use lower_wordlist.iter().enumerate().find()
// which is linear and has bad runtime performance

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut original = HashSet::new();
        let mut lower_wordlist = HashMap::new();
        let mut preprocessed = HashMap::new();
    
        wordlist.iter().for_each(|q| {
            original.insert(q);
            // lower_wordlist.push(q.to_lowercase());
            // preprocessed.push(Solution::preprocessing(q));
            lower_wordlist.entry(q.to_lowercase()).or_insert(q);
            preprocessed.entry(Solution::preprocessing(q)).or_insert(q);
        });

        let mut ret = vec![];
        queries.iter().for_each(|query| {
    
            if original.contains(query) {
                ret.push(query.clone());
                return;
            }

            let query_low = query.to_lowercase();
            if let Some (ori) = lower_wordlist.get(&query_low) {
                ret.push(ori.to_string());
                return;
            }

            let query_norm = Solution::preprocessing(query);
            if let Some(ori) = preprocessed.get(&query_norm)  {
                ret.push(ori.to_string());
                return;
            }
            // let low_match = lower_wordlist.iter().enumerate().find(|&(_, low_word)| {query_low == *low_word});
            // if let Some((idx, _)) = low_match {
            //      ret.push(wordlist[idx].clone());
            //     return;
            // }

            // let processed_query = Solution::preprocessing(query);
            // let matched_q = preprocessed.iter().enumerate().find(|&(_, word)| processed_query == *word);
            // if let Some((idx, _)) = matched_q {
            //     ret.push(wordlist[idx].clone());
            //     return;
            // }
        
            ret.push(String::from(""));
        });
        
        ret
    }

    fn preprocessing(ori: &str) -> String {
        ori.to_lowercase().chars().map(|ch| {
            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' { 'a' } else { ch }
        }).collect::<String>()
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test_vowel_replacement () {
        
    }
}