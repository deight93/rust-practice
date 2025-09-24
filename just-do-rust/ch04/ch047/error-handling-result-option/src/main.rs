// Rust가 제공하는 라이브러리에 있는 대부분의 함수는 리턴 타입이 Result 혹은 Option이다.
// 둘 다 열거형 타입니다. 이 두 타입에 대해서는 열거형을 다루는 챕터에서도 설명했는데, 에러 처리 방법에 초점을 맞춰 설명

// - esult는 정상적인 값에 대해서는 Ok(val), 에러인 경우에는 Err(e)로 표현된다.
// - Option는 값이 있을 경우에는 Some(val), 값이 없을 때는 None로 표현된다.
//
// 이 처럼 대부분의 이상 상황은 Result에서의 Err 혹은 Option에서의 None에 의해 처리

/*
다른 언어에서의 에러 처리

C에서는 에러처리에 대해서 언어 차원에서 가이드되는 것은 없다.
그러다 보니 관습상 굳어진 것이 에러의 경우 -1 혹은 0을 리턴하거나, NULL 이라는 것을 정의해놓고 이 NULL을 리턴하기도 한다.

Java에서는 Exception이라는 에러 처리용 클래스를 만들어서,
try~catch 구문에서 에러가 발생하면 Exception을 캐치해서 에러를 바로 처리하거나 상위 함수로 위임하거나 하는 방식이다.
 */

// 1. Err일 때 프로그램 종료하는 방식
// 아래 코드는 파일 오픈하는 함수인 File::open 함수가 리턴하는 Result를 처리하는 예다.
// "hello.txt" 파일이 없거나, 읽을 수 없거나 하면 에러가 발생하게되고, Err(e)가 리턴되게 된다.
// 여기서는 Err인 경우 패닉이 발생하도록 했다.
// 패닉을 발생시키는 매크로는 panic!(msg)이고 msg를 출력하고 스택을 해제한 후 프로그램을 종료

use std::{fs::File, io::Write};

fn main() {
    // let file = File::open("hello.txt");
    //
    // let file = match file {
    //     Ok(file) => file,
    //     Err(e) => {
    //         panic!("Error opening file: {:?}", e);
    //     }
    // };
    // println!("{:?}", file);

    // 2. Err일 때 메시지만 출력
    //foo.txt 파일을 생성하려고 노력해보고, 만약 에러가 나면 에러메시지를 출력하고 return하는 코드
    write_file("foo.txt");

    // 3. Option에 대한 처리
    // 아래 코드는 Option을 리턴하는 to_digit 메서드에 대한 처리 예제다.
    // to_digit에 의해 정상적으로 변환이 되었으면 변환된 값을 벡터에 넣고,
    // 비정상적인면 None이 리턴되기에 None에 대해서는 아무 작업도 안하고 루프를 계속 진행
    let num_str = "010-1234-5678";

    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        match c.to_digit(10) {
            Some(n) => v.push(n),
            None => (),
        }
    }
    println!("{:?}", v);

    // 4. Result, Option의 메서드에 의한 처리
    // 위에서의 예는 모두 Result와 Option으로 리턴될 때 match를 이용해서 정상으로 리턴되었는지
    // 에러 혹은 None으로 리턴되었는지를 판별하고, 판별된 것에 따른 루틴을 실행하는 방식을 사용했다.
    // 실제 코드를 짜다보면 이 방법보다는 unwrap, expect등 Result와 Option에서 제공되는 메서드들을 많이 이용
}

fn write_file(f_name: &str) {
    // foo.txt 파일을 생성하려고 노력해보고, 만약 에러가 나면 에러메시지를 출력하고 return하는 코드
    let mut f = match File::create(f_name) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    let _ = f.write_all(b"Hello, world!");
}
