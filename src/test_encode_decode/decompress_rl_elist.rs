/*
   https://leetcode.com/problems/decompress-run-length-encoded-list/
   1. 입력 받은 nums 배열의 pair는 "0,1", "2,3"과 같이 2*i, 2*i+1의 index의 값을 갖음
   2. 2*i+1의 index의 값이 2*i index의 값 만큼 output 배열에 추가 해야함
*/
pub fn test() {
    let nums = vec![1, 2, 3, 4];
    println!(
        "decompress_rl_elist result : {:?}",
        decompress_rl_elist(&nums)
    );
}

fn decompress_rl_elist(nums: &[u32]) -> Vec<u32> {
    nums.chunks(2).fold(vec![], |mut ans, chunk| {
        if let [freq, value] = chunk {
            ans.extend(
                std::iter::repeat(*value)
                    .take(usize::try_from(*freq).expect("The conversion of u32 to usize is safe.")),
            );
        }
        ans
    })
}

#[test]
fn tc() {
    let nums = vec![1, 2, 3, 4];
    let result = decompress_rl_elist(&nums);
    let check = vec![2, 4, 4, 4];
    assert_eq!(result, check);

    let nums = vec![1, 2, 3, 4, 5]; // 5는 panic 발생하지 않는지 테스트를 위해 추가
    let result = decompress_rl_elist(&nums);
    let check = vec![2, 4, 4, 4];
    assert_eq!(result, check);
}
