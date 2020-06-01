use crate::Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        // Transform the second dimension into a simple (i32, i32).
        let connections = connections
            .iter()
            .map(|v| (*v.first().unwrap(), *v.last().unwrap()))
            .collect::<Vec<(i32, i32)>>();

        // Create the forward graph.
        let forward_graph = connections.iter().fold(
            std::collections::HashMap::<i32, Vec<i32>>::with_capacity(n as usize),
            |mut acc, (k, v)| {
                acc.entry(*k).or_insert(Vec::new()).push(*v);
                acc
            },
        );

        // Create the backward graph.
        let backward_graph = connections.iter().fold(
            std::collections::HashMap::<i32, Vec<i32>>::with_capacity(n as usize),
            |mut acc, (k, v)| {
                acc.entry(*v).or_insert(Vec::new()).push(*k);
                acc
            },
        );

        // Queue of to-be-visited nodes.
        let mut v = Vec::with_capacity(n as usize);
        v.push(0);

        let mut result = 0;

        // Visited nodes.
        let mut visited = std::collections::HashSet::<i32>::with_capacity(n as usize);
        visited.insert(0);

        while !v.is_empty() {
            let current_node = v.pop().unwrap();

            // Grow visited set from the forward graph.
            // Increase result by 1 for every node.
            if let Some(forward) = forward_graph.get(&current_node) {
                for node in forward {
                    if visited.insert(*node) {
                        result += 1;
                        v.push(*node);
                    }
                }
            }

            // Grow visited set from the backward graph.
            // For each node, grow outward into the non-visited nodes.
            if let Some(backward) = backward_graph.get(&current_node) {
                for node in backward {
                    if visited.insert(*node) {
                        v.push(*node);
                    }
                }
            }
        }

        result as i32
    }
}

#[test]
pub fn test_contest_5426() {
    let input_1 = 6;
    let input_2 = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    assert_eq!(3, Solution::min_reorder(input_1, input_2));
}

#[test]
pub fn test_contest_5426_1() {
    let input_1 = 5;
    let input_2 = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
    assert_eq!(2, Solution::min_reorder(input_1, input_2));
}

#[test]
pub fn test_contest_5426_3() {
    let input_1 = 3;
    let input_2 = vec![vec![1, 0], vec![2, 0]];
    assert_eq!(0, Solution::min_reorder(input_1, input_2));
}

#[test]
pub fn test_contest_5426_4() {
    let input_1 = 10;
    let input_2 = vec![
        vec![0, 1],
        vec![2, 1],
        vec![3, 2],
        vec![0, 4],
        vec![5, 1],
        vec![2, 6],
        vec![5, 7],
        vec![3, 8],
        vec![8, 9],
    ];
    assert_eq!(6, Solution::min_reorder(input_1, input_2));
}
