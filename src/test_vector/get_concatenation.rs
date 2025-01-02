/*
   https://leetcode.com/problems/concatenation-of-array
*/
pub fn test() {
    let nums = [1, 2, 1];
    let ans = get_concatenation(&nums);
    println!("get_concatenation result : {ans:?}");
}

fn get_concatenation(nums: &[i32]) -> Vec<i32> {
    [nums, nums].concat()
}

#[test]
fn tc() {
    let nums = [1, 2, 1];
    let result = get_concatenation(&nums);
    let check = vec![1, 2, 1, 1, 2, 1];
    assert_eq!(result, check);
}
