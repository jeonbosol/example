/*
   https://leetcode.com/problems/running-sum-of-1d-array
*/
pub fn test() {
    let nums = [1, 2, 3, 4];
    println!("running_sum result : {:?}", running_sum(&nums));
}

pub fn running_sum(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |temp, &num| {
            *temp += num;
            Some(*temp)
        })
        .collect()
}

#[test]
fn tc() {
    let nums = [1, 2, 3, 4];
    let result = running_sum(&nums);
    let check = vec![1, 3, 6, 10];
    assert_eq!(result, check);
}
