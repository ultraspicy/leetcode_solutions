use std::collections::VecDeque;

use super::Solution;

impl Solution {
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let n = n as usize;
        let label = label.chars().collect::<Vec<char>>();
        if n == 1 {
            return 1
        }
        let mut graph =  vec![vec![]; n];
        edges.iter().for_each(|edge| {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
           
        });

        let mut visited =  vec![vec![vec![false; n]; n]; (1<<14) as usize];
        let mut q: VecDeque<_> = VecDeque::new();
        // all palindrom with len = 1
        (0..n).for_each(|node| {
                let mask = 1<<node;
                visited[mask][node][node] = true; 
                q.push_back((node, node, 1, mask));
            });
            
        // all palindrom with len = 2
        (0..n).for_each(|node| {
                graph[node].iter()
                    .map(|&second_node| {
                        (second_node, label[second_node])
                    })
                    .filter(|&(second_node, ch)| ch == label[node] && node < second_node)
                    .for_each(|(second_node, _)|{
                        let mask = (1<<node) + (1<<second_node);
                        visited[mask][node][second_node] = true; 
                        q.push_back((node, second_node, 2, mask));
                    });
            });
        
        let mut ret: i32 = 1;
        if q[q.len() - 1].0 != q[q.len() - 1].1 {
            ret = 2;
        }
        while !q.is_empty() {
            if let Some((start, end, len, mask)) = q.pop_front() {
                for i in 0..graph[start].len() {
                    if  (mask & 1<<graph[start][i]) != 0 {
                        continue;
                    } 
                    for j in 0..graph[end].len() {
                        if label[graph[start][i]] == label[graph[end][j]]
                            && graph[start][i] != graph[end][j]
                            && (mask & 1<<graph[end][j]) == 0 {
                                let new_mask = mask + (1<<graph[start][i]) + (1<<graph[end][j]);
                                let (first, second) = (graph[start][i].min(graph[end][j]), graph[start][i].max(graph[end][j]));
                                if !visited[new_mask][first][second] {
                                    visited[new_mask][first][second] = true;
                                    q.push_back((first, second, len + 2, new_mask));
                                    ret = ret.max(len + 2);
                                }     
                        }
                    }
                }
            }
        }

        ret       
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::max_len(3, vec![vec![0,1],vec![1,2]], String::from("aba"));
        assert_eq!(3, ret);
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::max_len(3, vec![vec![0,1],vec![0,2]], String::from("abc"));
        assert_eq!(1, ret);
    }

    #[test]
    fn unit_test3() {
        let ret = Solution::max_len(3, vec![vec![0,1],vec![1,2], vec![2,0]], String::from("hjj"));
        assert_eq!(3, ret);
    }
}
