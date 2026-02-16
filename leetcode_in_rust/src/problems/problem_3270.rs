use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3270
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let str1 = format!("{:04}", num1);
        let str2 = format!("{:04}", num2);
        let str3 = format!("{:04}", num3);

        let mut ret = String::new();
        for i in 0..4 {
            let max_char = str1.chars().nth(i).unwrap().min(str2.chars().nth(i).unwrap().min(str3.chars().nth(i).unwrap()));
            ret.push(max_char);
        }
        match ret.parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}
