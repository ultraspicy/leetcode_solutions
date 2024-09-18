use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3280
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn convert_date_to_binary(date: String) -> String {

        date.split('-').map(|single_date| match Self::str_to_binary(single_date){
            Ok(binary_date) => binary_date,
            Err(_error) => String::from("0000"),
        }).collect::<Vec<String>>().join("-")
    }

    fn str_to_binary(s: &str) -> Result<String, std::num::ParseIntError> {
        let number = s.parse::<u32>()?;
        Ok(format!("{:b}", number))
    }
}
