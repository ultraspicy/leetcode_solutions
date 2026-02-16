use super::Solution;

impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let n1 = n * n;
        let n2 = n1 * n;        

        let mut base16 = Self::to_base_n(n1, 16);
        let base36 = Self::to_base_n(n2, 36);

        base16.extend(base36);

        base16.into_iter().collect::<String>()
    }

    fn to_base_n (n: i32, base: i32) -> Vec<char> {
        let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();

        let mut n = n;
        let mut ret = Vec::new();

        while n != 0 {
            ret.push(alphabet[(n % base) as usize]);
            n = n / base;
        }
        
        ret.reverse();
        ret
    }
}
