/*
   https://leetcode.com/problems/kids-with-the-greatest-number-of-candies
   1. candies 배열에서 가장 큰 값을 구하기
   2. 배열을 순회하며 extra_candies 값을 더했을때 1에서 구한 값과 비교하여 작을때는 false 그 이외에는 true
*/

pub fn test() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    println!(
        "kids_with_candies result : {:?}",
        Some(kids_with_candies(&candies, extra_candies))
    );

    let candies = vec![4, 2, 1, 1, 2];
    let extra_candies = 1;
    println!(
        "kids_with_candies result : {:?}",
        Some(kids_with_candies(&candies, extra_candies))
    );

    let candies = vec![12, 1, 12];
    let extra_candies = 10;
    println!(
        "kids_with_candies result : {:?}",
        Some(kids_with_candies(&candies, extra_candies))
    );
}

fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Option<Vec<bool>> {
    candies.iter().max().map(|greatest| {
        candies
            .iter()
            .map(|candy| (*candy + extra_candies) >= *greatest)
            .collect()
    })
}

#[test]
fn tc() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let result = kids_with_candies(&candies, extra_candies);
    let check = vec![true, true, true, false, true];
    assert_eq!(result, Some(check));
}
