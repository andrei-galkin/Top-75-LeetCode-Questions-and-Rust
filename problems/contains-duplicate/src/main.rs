fn main() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1,2,3,4]), false);
    assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut _numbers = nums;
    _numbers.sort();

    for i in 0..(_numbers.len() - 1) {
        if _numbers[i] == _numbers[i + 1] {
            return true;
        }
    }

    return false;
}
