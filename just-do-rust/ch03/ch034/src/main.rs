// 이것은 주석으로, 프로그램에서 실행되지 않는 부분. 코드 설명용으로 사용
/*
C나 Java 처럼 /* ... */ 와 같이 주석을 처리할 수도 있다.
그러나 /** ... */ 는 허용하지 않는다.
 */
fn main() {
    let arg = 5;
    let answer = add_one(arg);
    println!("{}", answer);
}

/// 주어진 수에 1을 더해서 리턴한다.
///
/// # 사용법
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
