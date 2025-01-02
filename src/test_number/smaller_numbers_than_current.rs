/*
   https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
   순회하며 현재의 num의 값보다 작은 값이 있을 경우에 대한 카운팅
*/
use std::collections::HashMap;

pub fn test() {
    let nums = vec![8, 1, 2, 2, 3];
    println!(
        "smaller_numbers_than_current : {:?}",
        smaller_numbers_than_current(&nums)
    );
    println!(
        "smaller_numbers_than_current_perf : {:?}",
        smaller_numbers_than_current_perf(&nums)
    );

    // let nums = vec![6,5,4,8];
    // println!("{:?}", smaller_numbers_than_current(&nums));
    // println!("{:?}", smaller_numbers_than_current_perf(&nums));
    //
    // let nums = vec![7,7,7,7];
    // println!("{:?}", smaller_numbers_than_current(&nums));
    // println!("{:?}", smaller_numbers_than_current_perf(&nums));
}

fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|num| nums.iter().filter(|n| *n < num).count())
        .collect()
}

/*
   성능 최적화
   정렬된 배열을 이용하여 현재 값에 위치한 index 번호를 기준으로 갯수를 체크
*/
fn smaller_numbers_than_current_perf(nums: &[i32]) -> Vec<usize> {
    let mut map = HashMap::new();
    let mut ordered_nums = nums.to_owned();

    ordered_nums.sort_unstable();

    for (index, num) in ordered_nums.iter().enumerate() {
        map.entry(num).or_insert(index);
    }

    nums.iter()
        .map(|&num| *map.get(&num).unwrap_or(&0))
        .collect()
}

#[test]
fn tc() {
    let nums = vec![8, 1, 2, 2, 3];
    let result = smaller_numbers_than_current(&nums);
    let check = vec![4, 0, 1, 1, 3];
    assert_eq!(result, check);
}
