use std::collections::btree_map;

use itertools::Itertools;

use crate::problems::interview_question_bank::oai_timebase_kv_store::TimeMap;

impl TimeMap {
    const START: &str = "{";
    const END: &str = "}";
    fn json_serialize(&self) -> String {
        let mut s = String::new();
        s.push_str(Self::START);
        let key_treemap = self.map.iter().map(|(key, btree_map)| {
            let mut outer_kv = String::new();

            outer_kv.push_str(key);
            outer_kv.push_str(":");

            outer_kv.push_str(Self::START);
            let v = btree_map.iter().map(|(inner_k, inner_v)| {
                format!("{}:{}", inner_k, inner_v)
            }).join(",");
            outer_kv.push_str(&v);
            outer_kv.push_str(Self::END);


            outer_kv
        }).collect::<Vec<String>>().join(",");
        s.push_str(&key_treemap);
        s.push_str(Self::END);
        s
    }
}

#[cfg(test)]
mod serialization_test {
    use super::TimeMap;

    #[test]
    fn custom_serialization_deserialization() {
        let mut map = TimeMap::new();
        map.set("key1".to_string(), "value1".to_string(), 1);
        map.set("key1".to_string(), "value2".to_string(), 2);
        map.set("key3".to_string(), "value3".to_string(), 1);

        println!("{}", map.json_serialize());
        println!("{:?}", map);
    }

    #[test]
    fn serialization_deserialization() ->  Result<(), Box<dyn std::error::Error>> {
        let mut map = TimeMap::new();
        map.set("key1".to_string(), "value1".to_string(), 1);
        map.set("key1".to_string(), "value2".to_string(), 2);
        map.set("key3".to_string(), "value3".to_string(), 1);

        // json
        let s = serde_json::to_string(&map)?;
        println!("{}", s);
        let m_de = serde_json::from_str::<TimeMap>(&s)?;
        assert_eq!("value1", m_de.get(String::from("key1"), 1));

        // bincode
        let bin_s = bincode::serialize(&map)?;
        println!("{:?}", bin_s);
        let m_de_bin = bincode::deserialize::<TimeMap>(&bin_s)?;
        assert_eq!("value1", m_de_bin.get(String::from("key1"), 1));
        Ok(())
    }
}
