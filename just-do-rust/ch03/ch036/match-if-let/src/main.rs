fn main() {
    // 1 변수 값에 따른 matching
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 2 변수 범위에 따른 matching
    // match 패턴 안에 <=, < 같은 비교 연산자는 직접 못 씁니다.
    // 패턴 자리에 올 수 있는 건 리터럴, 범위(.., ..=), 바인딩, OR(|) 등입니다.
    let age: u32 = 33;
    let group = match age {
        0..=10 => "baby",
        11..=20 => "teen",
        21..=60 => "adult",
        _ => "old",
    };
    println!("{}", group);

    //3 변수 타입에 따른 matching
    let c = '5';
    // let num = c.to_digit(10).unwrap_or_else(|| 0);
    let num = match c.to_digit(10) {
        Some(n) => n,
        None => 0,
    };
    println!("{}", num);

    // 4 튜플에 대한 matching
    let n = 4;
    match (n % 3, n % 5) {
        (0, 0) => println!("fizzbuzz"),
        (0, _) => println!("fizz"),
        (_, 0) => println!("buzz"),
        (_, _) => println!("{} {} ", n % 3, n % 5),
    }

    // 5 예제3 -> if let
    /*
    let c = '5';
    let num = match c.to_digit(10) {
        Some(n) => n,
        None => 0,
    };
    println!("{}", num);
     */
    let c = '5';
    if let Some(num) = c.to_digit(10) {
        println!("{}", num);
    }
    // if let의 표현식은 if let <패턴> = <표현식> { do_something(); }
    // <표현식>이 <패턴>과 같을 때에만 do_something()을 수행한다.

    // 6 if let ~ else
    let c = '5';
    if let Some(n) = c.to_digit(10) {
        println!("{}", n);
    } else {
        println!("not a digit");
    }
}
