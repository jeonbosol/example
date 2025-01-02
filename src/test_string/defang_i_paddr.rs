/*
   https://leetcode.com/problems/defanging-an-ip-address/
   문자 변환 "." with "[.]"
*/
pub fn test() {
    let address = "1.1.1.1".to_string();
    println!("defang_i_paddr result : {}", defang_i_paddr(&address));
}

fn defang_i_paddr(address: &str) -> String {
    address.replace('.', "[.]")
}

#[test]
fn tc() {
    let address = "1.1.1.1".to_string();
    let result = defang_i_paddr(&address);
    let check = "1[.]1[.]1[.]1".to_string();
    assert_eq!(result, check);
}
