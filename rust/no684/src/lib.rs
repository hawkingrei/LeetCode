use std::collections::HashMap;

fn find(value: i32, map: &HashMap<i32, i32>) -> i32 {
    if map.len() == 0 {
        return value.clone();
    }
    if let Some(data) = map.get(&value) {
        if data == &value {
            return value;
        }
        find(*data, map)
    } else {
        value.clone()
    }
}
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();
    let mut data = vec![];
    for edge in &edges {
        let left = find(edge[0], &result);
        let right = find(edge[1], &result);
        if left == right {
            data = edge.to_vec();
        }
        if left > right {
            result.insert(left, right);
        } else {
            result.insert(right, left);
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use crate::find_redundant_connection;
    #[test]
    fn it_works() {
        assert_eq!(
            find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
