use super::Solution;

impl Solution {
    pub fn min_time_x(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {

        #[derive(Debug)]
        struct DSU {
            root: Vec<i32>,
            rank: Vec<usize>,
        }

        impl DSU {
            fn new (n: i32) -> Self {
                DSU { 
                    root: (0..n).collect(),
                    rank: vec![0; n as usize],
                }
            }

            fn find(&mut self, x: i32) -> i32 {
                if self.root[x as usize] != x {
                    self.root[x as usize] = self.find(self.root[x as usize])
                }

                self.root[x as usize]
            }

            fn union(&mut self, x: i32, y: i32) {
                let root_x = self.find(x) as usize;
                let root_y = self.find(y) as usize;

                if self.rank[root_x] > self.rank[root_y] {
                    self.root[root_y] = root_x as i32;
                } else if self.rank[root_x] < self.rank[root_y] {
                    self.root[root_x] = root_y as i32;
                } else {
                    self.root[root_y] = root_x as i32;
                    self.rank[root_y] += 1;
                }
            }
        }

        // state vars
        let mut dsu = DSU::new(n);
        let mut num_of_connected_component = n;

        let mut sorted_edges = edges.clone();
        sorted_edges.sort_by(|a, b| {
            b[2].cmp(&a[2])
        });

    
        let candidate = sorted_edges.iter().find_map(|edge|{
            let v1 = edge[0];
            let v2 = edge[1];
            let root_of_v1 = dsu.find(v1);
            let root_of_v2 = dsu.find(v2);
            dsu.union(root_of_v1, root_of_v2);

            if root_of_v1 != root_of_v2 {
                num_of_connected_component -= 1;   
            }
            if num_of_connected_component < k {
                Some(edge[2])
            } else {
                None
            }
        });

        match candidate {
            Some(ret) => ret,
            None => 0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_time_x(2, vec![vec![0, 1, 3]], 2);
        assert_eq!(ret, 3);

        let ret = Solution::min_time_x(3, vec![vec![0, 1, 2], vec![1, 2, 4]], 3);
        assert_eq!(ret, 4)
    }
}
