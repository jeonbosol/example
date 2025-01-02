/*
    https://leetcode.com/problems/count-of-matches-in-tournament/
    1. 전달 받은 수를 2로 나누고, 몫은 매칭팀, 주어진 팀에서 몫을 뺀 값을 advance로 지정
    2. 매칭된 팀의 합계를 계산
*/

pub fn test() {
    let n = 7;
    println!("number_of_matches result : {}", number_of_matches(n));
}

fn number_of_matches(mut n: i32) -> i32 {
    let mut ans = 0;

    while n > 1 {
        ans += n / 2;
        n = (n + 1) / 2;
    }

    ans
}

#[test]
fn tc() {
    let n = 7;
    let check = 6;
    let result = number_of_matches(n);

    assert_eq!(result, check);
}
