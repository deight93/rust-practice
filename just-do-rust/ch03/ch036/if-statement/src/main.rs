fn main() {
    let n = 5;

    // 전형적인 if-else 표현
    if n > 5 {
        println!("n is greater than 5");
    } else {
        println!("n is less than 5");
    }

    // 한줄 표현
    if n > 5 {
        println!("n is greater than 5");
    } else {
        println!("n is less than 5");
    }

    // 에러
    // if n { println!("n is true"); }

    // if가 조건식임을 이용
    let condition = if n > 5 { n + 1 } else { n - 1 };
    println!("condition is {}", condition);

    // if -> else if -> else
    let n = 6;

    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3"); //이게 출력된다.
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3, or 2");
    }
}
