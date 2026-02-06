use std::collections::VecDeque;
use std::collections::HashSet;

struct RideSharingSystem {
    drivers: VecDeque<i32>,
    riders: VecDeque<i32>,
    cancelled_riders: HashSet<i32>,
    requested_riders: HashSet<i32>,
}

#[allow(dead_code)]
impl RideSharingSystem {

    fn new() -> Self {
        RideSharingSystem { 
            drivers: VecDeque::new(), 
            riders: VecDeque::new(), 
            cancelled_riders: HashSet::new(), 
            requested_riders: HashSet::new(),
        }
    }
    
    fn add_rider(&mut self, rider_id: i32) {
        self.riders.push_back(rider_id);
        self.requested_riders.insert(rider_id);
    }
    
    fn add_driver(&mut self, driver_id: i32) {
        self.drivers.push_back(driver_id);
    }
    
    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        while let Some(first_rider) = self.riders.pop_front() {
            if !self.cancelled_riders.contains(&first_rider) {
                // rider is available 
                if let Some(first_driver) = self.drivers.pop_front() {
                    self.requested_riders.remove(&first_rider);
                    return vec![first_driver, first_rider];
                } else {
                    self.riders.push_front(first_rider);
                    return vec![-1, -1];
                }
            }
        }
        vec![-1, -1]
    }
    
    fn cancel_rider(&mut self, rider_id: i32) {
        if self.requested_riders.contains(&rider_id) {
            self.cancelled_riders.insert(rider_id);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let mut s = RideSharingSystem::new();
        s.add_driver(3);
        s.add_driver(2);
        s.add_rider(1);
        let matched_pair = s.match_driver_with_rider();
        print!("{:?}", matched_pair);
        s.add_driver(5);
        s.cancel_rider(3);
        let matched_pair = s.match_driver_with_rider();
        print!("{:?}", matched_pair);
        let matched_pair = s.match_driver_with_rider();
        print!("{:?}", matched_pair);
    }

}