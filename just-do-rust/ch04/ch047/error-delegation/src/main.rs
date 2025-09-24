/*
대규모의 프로그램을 짤 때는, 에러를 어떻게 처리할 것인지에 대한 전략을 세우로 짜야한다.

대부분의 경우 이러한 전략이 유효하다.

작성되는 모든 함수 혹은 메서드는 에러가 발생할 가능성이 있거나,
값이 없을 수 있다면 무조건 Result 혹은 Option을 리턴 타입으로 해서 작성한다.
하위 함수의 리턴값이 Result일 때, Result::Err에 대한 처리는 상위 함수로 위임한다.

즉, 현재의 함수가 실제 화면 출력 혹은 GUI를 담당하는 함수가 아니라면, 더 상위의 함수로 에러 처리를 위임한다.
최상위 함수 혹은 화면출력을 하는 함수에서 밑에서 부터 올라온 에러를 처리한다.
즉, 에러에 대해서 출력만 하던지 패닉 처리를 하던지 하는 것을 결정한다.
 */

// 위와 같이 에러의 처리를 최상위 함수를 하는 것은, 에러 처리에 대한 전략 관리를 수월하게 하기 위해서다.
// 이렇게 하지 않고, 개별 함수에서 에러가 발생할 때마다 에러 메시지를 출력하거나 패닉을 일으키거나 하게되면,
// 나중에 에러 처리에 대한 정책이 바뀌거나 하면 그 부분을 찾아서 수정하기가 어렵기 때문이다.

// 에러를 상위로 전파하는 키워드는 ?다. 아래 예제는 ?를 이용해서 에러를 상위로 위임하는 예제

use std::fs::File;
use std::io::{Error, Read};

fn read_name() -> Result<String, Error> {
    // read_name 함수의 리턴 타입을 Result로 했다.
    let mut name = String::new();
    File::open("name.txt")?.read_to_string(&mut name)?;
    // File::open("name.txt")?은 성공하면 파일 핸들러가 Ok(f)로 리턴되고,
    // 실패해서 Err가 나오면 이 Err을 리턴값으로 해서 read_name 함수를 종료시킨다.
    // 여기도 ?가 붙어 있어서, 만약 read_to_string 메서드에서 Err가 리턴되면
    // 이것을 리턴값으로해서 read_name함수를 종료
    Ok(name)
}

fn main() {
    let result = match read_name() {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            String::from(" ")
        }
    };
    println!("{}", result);
}
