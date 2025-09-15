/*
fn 함수이름(파라미터1 이름: 파라미터1 타입, 파라미터2 이름: 파라미터2 타입) -> 리턴 타입 {
  ...
}
 */

fn main() {
    println!("Hello, world!");

    let c = add(3, 5);
    println!("c={}", c);

    let d = is_even1(3);
    println!("d={}", d);

    let e = is_even2(4);
    println!("e={}", e);
}

fn add(a: i32, b: i32) -> i32 {
    // 파라미터
    return a + b; //return a+b; 라고 하지 않고 `a+b`라고 세미콜론 없이 사용해도 된다.
}

/*
Rust에서는 ;로 끝나는 것을 statement(문)라고 하고, ;가 없는 것은 expression(식)이라고 한다.
statement(문): ;로 끝난다. let a = 5;
expression(식): ;가 없다. a+b
 */

fn is_even1(n: u32) -> bool {
    return n % 2 == 0;
}

fn is_even2(n: u32) -> bool {
    n % 2 == 0
}
