use super::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut ret: Vec<char> = Vec::new();
        let mut count = 0;
        s.chars()
            .for_each(|c| {
                if c == '(' {
                    count += 1;
                    stack.push(c);
                } else {
                    count -= 1;
                    stack.push(c);
                }
                if count == 0 {
                    let a = stack.iter().skip(1).take(stack.len() - 2).collect::<Vec<_>>();
                    ret.extend(a);
                    stack = Vec::new(); 
                }
            });
        ret.iter().collect::<String>()
    }

}
