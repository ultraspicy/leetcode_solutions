use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, vec};

use super::Solution;

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        #[derive(Debug)]
        struct DSU {
            parent: Vec<i32>,
            rank: Vec<i32>,
        }

        impl DSU {
            fn new (size: i32) -> Self {
                DSU {
                    parent: (0..size).collect(),
                    rank: vec![0; size as usize],
                }
            }
 
            fn find(&mut self, x: i32) -> i32 {
                if self.parent[x as usize] != x {
                    self.parent[x as usize] = self.find(self.parent[x as usize]);
                }
                self.parent[x as usize]
            }

            fn union(&mut self, x: i32, y: i32) {
                let root_x = self.find(x);
                let root_y = self.find(y);

                if self.rank[root_x as usize] < self.rank[root_y as usize] {
                    self.parent[root_x as usize] = root_y; 
                } else if self.rank[root_x as usize] > self.rank[root_y as usize] {
                    self.parent[root_y as usize] = root_x; 
                } else {
                    self.parent[root_x as usize] = root_y; 
                    self.rank[root_y as usize] += 1;
                }
            }
        }

        // build connected component
        // id has offset 
        let mut connect_components = DSU::new(c);

        connections.iter().for_each(|v| {
            connect_components.union(v[0] - 1, v[1] - 1);
        });
        // org root -> connected_component
        let mut root_to_grid: HashMap<i32, Vec<i32>> = HashMap::new();
        (0..c).collect::<Vec<_>>().iter()
            .for_each(|&id| {
                root_to_grid.entry(connect_components.find(id)).or_insert_with(Vec::new).push(id);
            });
        // build min heap 
        let mut root_to_grid_heapified: HashMap<i32, BinaryHeap<_>> = HashMap::new();

        root_to_grid.into_iter()
            .for_each(|(k, vec)| {
                let mut h = BinaryHeap::new();
                vec.into_iter().for_each(|v| {
                    h.push(Reverse(v));
                });
                root_to_grid_heapified.insert(k, h);
            });

        let mut ret: Vec<i32> = vec![];
        let mut status = vec![true; c as usize];
        
        queries.iter().for_each(|query| {
            let query_type = query[0];
            let query_value = query[1] - 1;
            match query_type {
                1 => {
                    if status[query_value as usize] {
                        ret.push(query_value + 1);
                        return;
                    }
                    let h = root_to_grid_heapified
                        .get_mut(&connect_components.find(query_value)).unwrap();
                    while let Some(&Reverse(top)) = h.peek() {
                        if status[top as usize] {
                            ret.push(top + 1);
                            break;
                        } else {
                            h.pop();
                        }
                    }
                    if h.is_empty() {
                        ret.push(-1);
                    }
                }
                _ => {
                    status[query_value as usize] = false;
                }
            }
        });

        ret
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let connections = vec![
            vec![1,2],
            vec![2,3],
            vec![3,4],
            vec![4,5],
        ];
        let queries  = vec![
            vec![1,3],
            vec![2,1],
            vec![1,1],
            vec![2,2],
            vec![1,2],
        ];

        let result = Solution::process_queries(5, connections, queries);
        assert_eq!(result, vec![3, 2, 3])
    }
}