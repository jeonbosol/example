/*
   https://leetcode.com/problems/shuffle-string/
   indices의 값이 sorting의 기준이 됨
   indices를 기준으로 s 문자열을 sorting
   - zip, sort_by, cpm 기능을 이용
*/

pub fn test() {
    let s = String::from("codeleet");
    let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];

    println!("restore_string result : {}", restore_string(&s, indices));
}

fn restore_string(s: &str, indices: Vec<i32>) -> String {
    let mut s_array: Vec<(i32, char)> = indices.into_iter().zip(s.chars()).collect();
    s_array.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    s_array.into_iter().map(|(_a, b)| b).collect()
}

#[test]
fn tc() {
    let s = String::from("codeleet");
    let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];
    let result = restore_string(&s, indices);
    let check = "leetcode".to_string();
    assert_eq!(result, check);
}
