use super::Solution;

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut parsed = transactions.iter().map(|fat_tx|{
            let mut split = fat_tx.split(",");
            let customer_name = split.next().unwrap();
            let time = split.next().expect("cannot parse time string from fat_tx, missing data").parse::<i32>().expect("invalid time format");
            let amount = split.next().expect("cannot parse amount string from fat_tx, missing data").parse::<i32>().expect("invalid amount format");
            let city = split.next().unwrap();
            let flag = if amount > 1000 { true } else { false };

            (customer_name, time, amount, city, flag)
        }).collect::<Vec<(&str, i32, i32, &str, bool)>>();

        for i in 0..parsed.len() {
            for j in (i + 1)..parsed.len() {
                let (tx1, tx2) = (parsed[i], parsed[j]);
                if tx1.0 == tx2.0 && (tx1.1 - tx2.1).abs() <= 60 && tx1.3 != tx2.3 {
                    parsed[i].4 = true;
                    parsed[j].4 = true;
                }
            }
        }
        
        parsed.into_iter().filter(|parsed_tx| {
            parsed_tx.4
        }).map(|tx| {
            format!("{},{},{},{}", tx.0,  tx.1,  tx.2,  tx.3)
        }).collect::<Vec<String>>()

    }
}

