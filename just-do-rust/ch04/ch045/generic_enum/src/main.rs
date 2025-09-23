/*
Rust에서 기본 제공하는 열거형에서의 제네릭 타입
Rust에서 가장 많이 사용되는 열거형은 Option과 Result다. 둘 다 제네릭 타입으로 정의 되어 있다.

    pub enum Option<T> {
        None,
        Some(T),
    }

    enum Result<T, E> {
       Ok(T),
       Err(E),
    }
Option의 경우는 1개의 제네릭 타입, Result는 두 개의 제네릭 타입이 사용된다.
 */

// Option 제네릭

// 열거형 Option은 None과 Some을 열거타입으로 가지고 있다.
// 값이 없을 때는 None으로, 값이 있을 때는 Some에 값을 넣어 리턴할 때 쓰인다.
// 제네릭이 사용된 곳은 Some이다. Some은 제네릭 타입 T를 값으로 갖는 단순한 구조체다.

fn divide(numerator: i32, demominator: i32) -> Option<i32> {
    // 두 수를 나눈 몫을 리턴하는 함수 divide에서, 나누는 값이 0이면 None을,
    // 아니면 그 몫을 Some에 담아서 리턴하는 것이다. 사용되는 타입은 i32다.
    if demominator == 0 {
        None
    } else {
        Some(numerator / demominator)
    }
}

fn divide2(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

/*
tip

위 예제에서 divide 함수의 파라미터를 <T>로 해서 구현을 하지 않았다.
이유는 T 타입인 denominator에 대해서 if denominator == 0을
처리하려면 == 0이라는 것을 T 타입에 대해 가능하게 해야하기 때문이다.
이게 어떤 의미인가 하면, i32 등 정수형에 대해는 == 0 연산이 가능하고,
f64 등 부동소숫점 타입의 경우는 == 0.0 연산이 가능할 것이다.
즉, 정수형인지 부동소숫점 타입 인지에 따라 == 0일지 == 0.0으로 할 지를 판단해야하고,
만약 String 타입의 경우나 bool 타입의 경우는 0과 비교를 하지 않고 None으로 처리해야한다.
이처럼 0과 같은 지를 비교하는 것이 제네릭 타입 T에 대해 너무 번거로운 일이기에 여기서는 구현을 하지 않았다.
만약 해야한다면 Zero라는 트레잇을 구현하고, 이 Zero를 T에 대해 트레잇 바운드로 한정시켜서 해야할 것이다.
 */

// Result 제네릭

// Result에 쓰인 제네릭 타입 예를 보자.
//
// Result에는 <T, E>라는 두 개의 제네릭 타입이 사용된다.
// T는 Ok<T>에 사용되고, E는 Err<E>에 사용된다.
// 즉, T 타입은 성공했을 때의 값에 대한 타입이고, E 타입은 에러일 때의 값에 대한 타입이다.
// <T, E>에 사용된 문자를 <X, Y> 처럼 다른 문자로 해도 전혀 문제 없다. 즉, 문자에 어떤 의미가 있는 것이 아니다.
use std::fs::File;

fn main() {
    let a = 26;
    let b: i32 = 5;
    match divide(a, b) {
        None => println!("oops!"),
        Some(n) => println!("{} / {}", a, b),
    }

    let a = 26.0;
    let b = 5.0;
    match divide2(a, b) {
        None => println!("can't divide by 0"),
        Some(x) => println!("result = {}", x),
    }

    let f = File::create("hello.txt");

    let f = match f {
        Ok(file) => file, //T 타입
        Err(error) => {
            //E 타입
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}
