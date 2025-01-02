/*
   https://leetcode.com/problems/jewels-and-stones/
   1. hashSet을 이용하여 jewels의 문자열을 저장
   2. stones의 문자열에서 1에 저장된 값에 포함되는 문자열 카운팅
*/
use std::collections::HashSet;

pub fn test() {
    let jewels = "aA".to_string();
    let stones = "aAAbbbb".to_string();
    println!(
        "num_jewels_in_stones result : {}",
        num_jewels_in_stones(&jewels, &stones)
    );

    let jewels = "z".to_string();
    let stones = "ZZ".to_string();
    println!(
        "num_jewels_in_stones result : {}",
        num_jewels_in_stones(&jewels, &stones)
    );
}

fn num_jewels_in_stones(jewels: &str, stones: &str) -> usize {
    let char_set: HashSet<char> = jewels.chars().collect();
    stones.chars().filter(|c| char_set.contains(c)).count()
}

#[test]
fn tc() {
    let jewels = "aA".to_string();
    let stones = "aAAbbbb".to_string();
    let result = num_jewels_in_stones(&jewels, &stones);
    let check = 3;
    assert_eq!(result, check);
}
