// 모든 콘솔 입력은, 콘솔 입력용 핸들러를 얻고 난 후 처리 가능하고,
// 콘솔 입력용 핸들러는 std::io::stdin()으로 얻을 수 있다.
// 즉, Rust에서의 콘솔 입력 처리는 std::io::stdin()을 통해 핸들러를 얻은 후, 이 핸들러를 통해 처리된다.

// 이후 이 핸들러를 그냥 사용할 것인지, lock을 걸어서 사용할 것이지를 결정해야한다.
// 여러 쓰레드가 경합하는 프로그램에서는 lock을 걸어서 사용하는 것이 좋다.
// 그렇지 않으면 다른 쓰레드에 의해 인터럽트 당할 수 있다.
//
// 여기서는 lock 없이 그냥 사용하는 경우를 먼저 설명하고, 그 다음 lock을 이용한 사용을 설명하겠다.

/*
  tip

  알고리즘 풀이 사이트 등에서 사용할 때는, lock을 사용하는 것이 좋다.
  왜냐하면, 자신의 작성한 코드가 알고리즘 풀이 사이트의 서버에서 동작되는 것이기에,
  다른 여러 사람들의 코드와 경합하면 처리된다.
  따라서, 이 경우 자신이 먼저 콘솔 핸들러를 얻은 후 lock을 걸지 않으면,
  중간에 콘솔 핸들러를 다른 쓰레드에 빼앗길 수도 있기 때문이다.
*/
use std::io::{self, BufRead};

fn main() {
    // 단일 쓰레드 환경에서의 입력 처리

    // 1. 한 줄을 String으로 받기

    // 콘솔 입력 핸들러는 std::io::stdin()으로 부터 얻을 수 있다.
    let stdin = std::io::stdin();
    // buf는 String 타입이고, 핸들러가 이 buf에 입력값을 쓸 것이기에 mut 처리해야 한다.
    let mut buf = String::new();
    // read_line(&mut buf)는 입력되는 값에 대해 개행 값(줄 바꿈 기호)을 만날 때까지의 문자열을 buf에 쓴다.
    // read_line 메서드의 리턴 타입은 Result다.
    // 따라서, match를 사용하거나, 여기서 처럼 expect를 써서 Ok에 대한 값을 얻어내야 한다.
    // expect 메서드는 Ok의 경우는 Ok가 함유하고 있는 값을 리턴하고,
    // Err의 경우는 expect에 넘겨진 문자열 파라미터를 출력하고 panic을 발생시킨다.
    stdin.read_line(&mut buf).expect("input error");
    // 그냥 Ok의 경우만 그 값을 받고, Err의 경우는 panic을 발생시키면서 프로그램을 종료하려할 때는 unwrap을 사용하면 된다.
    // stdin.read_line(&mut buf).unwrap();

    println!("buf: {}", buf);

    // 2. 한 줄에 있는 숫자값 n, k를 읽는 경우
    let stdin2 = std::io::stdin();
    let mut buf2 = String::new();
    stdin2.read_line(&mut buf2).expect("input error");

    // 문자열에서 어떤 문자를 기준으로 문자열을 분리하는 것은 split(ch) 메서드이다.
    // 문자 ch를 기준으로 문자열을 분리한다. 여기서는 공백을 기준으로 분리하는 것이기에 split(' ')과 같이 처리된다.
    // split으로 분리하고 난 후 리턴되는 것은 분리된 &str 들에 대한 iterator다.
    // 따라서 map같은 메서드를 이어서 사용할 수 있다. map은 분리된 각 &str 별로 실행이 되고,
    // map에 있는 클로저에서는 &str을 .to_string()을 통해서 String으로 변환한다.
    // 변환된 String 값들은 collect를 통해서 Collection으로 변환이 되는데,
    // 여기서는 let input:Vec<String>에서 Vec<String> 타입으로 지정했기에 String타입 벡터로 변환된다.
    let input: Vec<String> = buf2.split(" ").map(|x| x.to_string()).collect();
    // input에는 n과 k에 해당하는 문자열이 들어 있다.
    // 이것을 숫자형으로 바꿔야하는데, 먼저 양 가장자리에 공백이 있을 수 있기에 이를 없애는 trim()을 해주고,
    // 그 다음에 숫자로 바꾸는 parse()를 하면 된다.
    // parse()를 할 때는 어떤 숫자형 타입으로 변환하는지를 지정해줘야 하는데,
    // '터보피시(turbofish)'라고 불리는 형태로 변환 타입을 적어부면 된다.
    // 여기서는 usize로 변경하는 걸로 해서 parse::<usize>()와 같이 되어 있다.
    // 만약 i32 타입으로 변환되길 원한다면 parse::<i32>()로 하면된다.
    let n = input[0].trim().parse::<usize>().unwrap();
    // parse()의 리턴은 Result 타입이다. 따라서, unwrap() 등을 통해서 값을 추출해내야 한다.
    let k = input[1].trim().parse::<usize>().unwrap();

    println!("n: {}, k: {}", n, k);

    // 멀티 쓰레드 환경에서의 입력 처리

    /*
    콘솔 입력에 대한 핸들(handle)은 여러 쓰레드가 공유할 수 있는 글로벌 리소스다.
    따라서, 콘솔 입력으로 여러 줄을 읽는다던지 대량의 데이터를 읽을 때는, 핸들에 lock을 걸어서 사용하는 것이 좋다.
    그렇지 않으면 다른 쓰레드에 의해서 인터럽트 되서 입력 일부을 읽지 못할 수도 있다.

    핸들에 락을 거는 것은 lock() 메서드를 이용하면 된다.
    핸들을 얻고 바로 락을 해서 사용하면 되기에 std::io::stdin().lock()과 같이 하면 된다.
    그리고, lock을 얻어서 입력값을 읽는 경우 std::io::BufRead트레잇을 이용하게 된다.
    따라서 use std::io::{self, BufRead};와 같이 use 선언을 해 놓고 사용하면 편하다.
     */

    // 한 줄 입력 받기

    // 핸들에 lock을 걸고, 입력되는 한 줄을 읽어 보자. 사용되는 메서드는 read_line(buf)이다.
    // 이 메서드는 BufRead 트레잇에 정의되어 있고, lock()에 의해 리턴되는 StdinLock에 구현되어 있다.
    // 리턴 타입은 Result이고 Ok에 읽어들인 입력값의 길이가 들어가게되고, buf에 콘솔에서 입력된 문자열을 쓴다.
    // 위 코드는 콘솔로 입력되는 값을 캐리지 리턴이 있을 때까지 읽는다. 정확하게는 캐리지 리턴도 포함해서 읽는다.

    let mut buf = String::new();
    let len = io::stdin().lock().read_line(&mut buf).expect("input error");
    buf = buf.trim().to_owned();
    println!("buf: {}, len: {}", buf, len);
}
