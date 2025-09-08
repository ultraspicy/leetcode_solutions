use std::collections::HashMap;

struct Bank {
    balances: Vec<i64>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Bank { balances: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let n = self.balances.len() as i32;
        if account1 >= 1 && account1 <= n && account2 >= 1 && account2 <= n
            && self.balances[account1 as usize - 1] >= money {
                self.balances[account1 as usize - 1] -= money;
                self.balances[account2 as usize - 1] += money;
                return true;
        }
        false
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let n = self.balances.len() as i32;
        if account >= 1 && account <= n {
            self.balances[account as usize - 1] += money;
            return true;
        }
        false
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let n = self.balances.len() as i32;
        if account >= 1 && account <= n && self.balances[account as usize - 1] >= money {
            self.balances[account as usize - 1] -= money;
            return true;
        }
        false
    }
}
