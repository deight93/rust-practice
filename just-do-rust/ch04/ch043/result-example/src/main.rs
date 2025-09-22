// Result를 리턴하는 가장 좋은 예는 파일 핸들링이다.
// 파일을 오픈하려는데 없는 파일이라거나, 파일을 생성하려는데 생성이 안된다거나 하는 에러가 발생할 수 있는데,
// Rust에서는 이런 에러를 처리할 수 있게 관련한 함수의 리턴값을 Result로 하고 있다.

use std::{fs::File, io::Write};
// File 패키지에 파일 핸들링관련 메서드들이 있다. 사용하려면 use std::fs::File;를 해줘야 한다.
// f.write_all 메서드를 쓸려면 io::Write 패키지를 use해야하기 때문
fn main() {
    write_file("foo.txt");

    // 에러인 경우 그 에러를 출력하고, 에러가 아니면 파일 핸들러를 얻어내서
    // 그것을 이용하여 "hello"라는 문자열을 파일에 쓰게 하자.
    write_file_result("boo.txt");

    // 이 Result 타입을 처리하는 루틴을 추가
    match write_file_error("coo.txt") {
        Ok(()) => {
            println!("write file success");
        }
        Err(e) => {
            println!("write file error {:?}", e);
            return;
        }
    }

    // Result short
    match write_file_error_short("koo.txt") {
        Ok(()) => {
            println!("write file success")
        }
        Err(e) => {
            println!("write file error {:?}", e);
            return;
        }
    }

    // unwrap의 사용
    // 위 코드의 expect를 사용한 부분을 unwrap을 사용할 수도 있다.
    // unwrap을 쓰면 프로그래머가 지정하는 err_msg 없이, 에러를 발생시킨다.
    write_file_error_short("roo.txt").unwrap();
}

fn write_file(f_name: &str) {
    let mut f = File::create(f_name); // File::create(f_name) 에 의해 파일이 생성된다.

    // 두 가지 문제
    // 파일이 제대로 생성되었는지 확인이 안된다
    // . *.txt 같은 이상한 파일명을 보내면 파일이 생성되지도 않고, 파일이 생성되지 않았다는 메시지도 없다.
    // File::create(f_name)이 리턴하는 파일 핸들러를 가지고 해당 파일에 텍스트를 쓰던지 해야하는데,
    // File::create함수는 리턴값이 Result이다.
    // Result::Ok 안에 감싸있는 파일 핸들러를 뽑아내야 그 다음 작업이 가능하다.
}

fn write_file_result(f_name: &str) {
    // Result를 처리하기 위해 match를 사용
    let mut f = match File::create(f_name) {
        // Ok의 경우는 file을 f에 할당하고, Err의 경우는 에러내용을 출력하고 함수를 그냥 종료한다.
        Ok(file) => file,
        Err(error) => {
            // Err(error) =>에 대한 처리를 블록 {}으로 감싼 것에 유의.
            // 처리할 내용이 println();과 같이 구문(statement)이 있을 경우 블록으로 감싸주면 된다.
            println!("Error: {}", error);
            return;
        }
    };
    let _ = f.write_all(b"hello");
    // let _ =와 같이 해준 것은, 이렇게 안하면 warning이 발생하기 때문.
    // f.write_all(b"hello");이 뭔가를 리턴하는데 그것에 대해 "나는 아무것도 하지 않아요"라는 것을
    // 프로그래머가 명시해주길 원하기 때문. Rust의 프로그래밍 철학이다.
    // 최소한 프로그래머가 자신이 호출한 함수가 리턴값이 있는지 없는지 정도는 알고 있기를 바라는 것.

    // b"hello"에서 b를 붙여준것은, write_all이 파라미터로 &[u8]을 원하기 때문.
    // 더블 따옴표로 감싸진 문자열 앞에 b를 붙이면 &[u8] 타입이 된다.
}

// write_file에서 에러가 발생한 것에 대해서, 해당 함수 내부에서 에러를 출력하고 그냥 리턴했다.
// 이러지 말고 에러처리를 호출한 쪽에서 에러처리를 하도록 '에러 위임'을 해보자.
fn write_file_error(f_name: &str) -> Result<(), String> {
    // 함수에 대한 리턴값이 Result에 대한 처리를 했다.
    let mut f = match File::create(f_name) {
        // File::create 메서드에 대한 리턴 값이 Err(error)의 경우에는,
        // 해당에러를 호출한 함수로 보내기 위해서, 새로운 Err 인스턴스를 만들고 이를 리턴
        Ok(file) => file,
        Err(error) => {
            return Err(format!("Error: {}", error));
        }
    };
    let _ = f.write_all(b"world");
    return Ok(());
    // f.write_all까지 모두 수행 후 return Ok(())를 했다. 모든 수행을 성공했다는 리턴값이다.
    // 여기서 Ok의 인자로 ()가 들어감에 유의한다. 리턴하는 Result의 타입을 Result<(), String>로 했기 때문이다.
}

// 에러를 전파하는 숏컷 ?
// String이었으나, 여기서는 std::io::Error로 변경했다. ?에 의해 리턴되는 타입이 Error이기 때문
fn write_file_error_short(f_name: &str) -> Result<(), std::io::Error> {
    // match를 써서 Err의 경우, 새롭게 Err 인스턴스를 만들어서 리턴했던 코드를, 여기서는 ?를 써서 해결했다. 굉장히 간단
    /*
    여기서 ?의 의미는, File::create(f_name)이 수행되었을 때
    - 에러가 없으면 생성된 파일 핸들러를 f로 배정하고,
    - 에러가 나면 Error를 리턴하라는 의미
     */
    let mut file = File::create(f_name)?;
    let _ = file.write_all(b"world");
    return Ok(());
}

// ------------------------------------------------------------------
// Result를 처리하는 방법이 너무 복잡한 느낌이다.
// match를 써서도 할 수 있고, ?도 있고, expect와 unwrap도 있다.
//
// 복잡할 때는 원칙을 만들어두면 편리하다. 에러의 경우 다음과 같이 처리하도록 하자.

/*
에러 처리 관련 원칙 정리

1. 함수 리턴값으로 Result 사용할지?
   - 간단한 테스트/작은 프로그램: 굳이 Result 쓰지 않아도 됨.
   - 안정성이 중요한 큰 규모 프로그램: 반드시 Result 사용.

2. 에러 최종 처리 위치
   - 간단한 프로그램: 에러 발생 지점에서 처리.
   - 복잡한 프로그램: 최초 호출 지점에서 처리 (하위 함수는 에러를 위임).
   - UI 있는 프로그램: 최종 UI 단에서 처리.

3. Result 처리 방식 (match, ?, expect, unwrap)
   - match: 복잡하고 상용 프로그램일 때 사용.
   - ?: 에러 위임이 필요한 함수에서 사용.
   - expect / unwrap: 간단한 프로그램에서 사용.

정리:
- 간단한 프로그램 → expect 또는 unwrap
- 안정성이 요구되는 프로그램 → ? 로 위임하거나 match 로 처리
*/
