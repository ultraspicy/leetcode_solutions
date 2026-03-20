#[cfg(test)]
mod serialization_test {
    use crate::problems::interview_question_bank::oai_timebase_kv_store::TimeMap;

    #[test]
    fn serialization_deserialization() ->  Result<(), Box<dyn std::error::Error>> {
        let mut map = TimeMap::new();
        map.set("key1".to_string(), "value1".to_string(), 1);
        map.set("key1".to_string(), "value2".to_string(), 2);
        map.set("key3".to_string(), "value3".to_string(), 1);

        // 
        let s = serde_json::to_string(&map)?;
        println!("{}", s);
        let m_de = serde_json::from_str::<TimeMap>(&s)?;
        assert_eq!("value1", m_de.get(String::from("key1"), 1));

        let bin_s = bincode::serialize(&map)?;
        println!("{:?}", bin_s);
        let m_de_bin = bincode::deserialize::<TimeMap>(&bin_s)?;
        assert_eq!("value1", m_de_bin.get(String::from("key1"), 1));
        Ok(())
    }
}
