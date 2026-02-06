use std::{collections::{BinaryHeap, HashMap, HashSet}, hash::Hash};

struct AuctionSystem {
    // <item_id, heap<(amount, user_id)>>
    booking: HashMap<i32, BinaryHeap<(i32, i32)>>,
    // <user_id, Map<item_id, amount>>
    active_bid: HashMap<i32, HashMap<i32, i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionSystem {

    fn new() -> Self {
        AuctionSystem { 
            booking: HashMap::new(),
            active_bid: HashMap::new(),
         }
    }
    
    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        self.remove_bid(user_id, item_id);

        self.active_bid.entry(user_id).or_insert(HashMap::new()).insert(item_id, bid_amount);
        self.booking.entry(item_id).or_insert(BinaryHeap::new()).push((bid_amount, user_id));
    }
    
    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        self.add_bid(user_id, item_id, new_amount);
    }
    
    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        if let Some(item_id_then_amount) = self.active_bid.get_mut(&user_id) {
            if let Some(&amount) = item_id_then_amount.get(&item_id) {
                item_id_then_amount.remove(&item_id);
            }
        }
    }
    
    fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
        // for the item, get first active bid
        if let Some(heap) = self.booking.get_mut(&item_id) {
            while let Some((amount_in_q, user_id)) = heap.peek() {
                match self.active_bid.get(user_id) {
                    Some(item_amount) => {
                        match item_amount.get(&item_id) {
                            Some(latest_amount) => {
                                if *latest_amount == *amount_in_q {
                                    return *user_id;
                                } else {
                                    heap.pop();
                                }
                            },
                            None => {
                                heap.pop();
                            }
                        }
                    },
                    None => {
                        heap.pop();
                    },
                }
            }
        }
        -1
    }
}
