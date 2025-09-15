fn main() {
    /*
    정수
     */
    //1. 가독성을 위해 숫자 사이에 _ 사용 가능
    let a1 = 100_000;
    let a2 = 100000;
    println!("{} {}", a1, a2); //100000 100000

    //2. 16진수
    let b1 = 0xff;
    let b2 = 15 * 16 + 15;
    println!("{} {}", b1, b2); //255 255

    //3. 8진수
    let c1 = 0o77;
    let c2 = 7 * 8 + 7;
    println!("{} {}", c1, c2); //63 63

    //4. 이진수
    let d1 = 0b1111_0000;
    let d2 = 128 + 64 + 32 + 16;
    println!("{} {}", d1, d2); //240 240

    //5. 문자 아스키 값
    let e1 = b'A'; //e1:u8
    let e2 = 'A'; //e2:char
    println!("{} {}", e1, e2); //65 A

    /*
    부동 소수점
     */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x={}, y={}", x, y);

    let u: u32 = 40000;
    let sqrt_u = (u as f64).sqrt();
    println!("sqrt(u) = {}", sqrt_u);

    /*
    부울
     */
    let t1 = true;
    let t2: bool = false;
    println!("t1 = {}, t2 = {}", t1, t2);

    /*
    문자
     */
    let a = 'a';
    let z = 'z';

    println!("{}, {}", a, z); //a, z
}
