use std::collections::HashMap;

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3,2,4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3,3], 6), vec![0, 1]);
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in numbers.iter().enumerate() {
    
        let complement: i32 = target - num;
        
        if map.contains_key(&complement) {
            let n: i32 = *map.get(&complement).unwrap();
            return vec![n, i as i32];
        }

        map.insert(*num, i as i32);
    }

    return vec![];
}
