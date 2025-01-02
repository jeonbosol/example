/*
    https://leetcode.com/problems/create-target-array-in-the-given-order/
    1. target에는 파라미터로 넘어온 index위치에 nums의 값을 넣어야 함
    2. 같은 index위치에 값을 넣는 경우
*/
pub fn test() {
    let nums = [0, 1, 2, 3, 4];
    let index = [0, 1, 2, 2, 1];

    println!("{:?}", create_target_array(&nums, &index));
}

fn create_target_array(nums: &[i32], indices: &[usize]) -> Vec<i32> {
    let mut target_array = Vec::with_capacity(nums.len());

    for (&index, &value) in indices.iter().zip(nums.iter()) {
        target_array.insert(index, value);
    }

    target_array
}

#[test]
fn tc() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];

    let result = create_target_array(&nums, &index);
    let check = vec![0, 4, 1, 3, 2];

    assert_eq!(result, check);
}
