// 예를 들어, file_backup.exe라는 프로그램이 있을 때
// file_backup "C:/src_dir" "D:/tgt_dir 처럼 프로그램을 실행한다면,
// "C:/src_dir"과 "D:/tgt_dir"를 입력으로 받아서 프로그램에 전달해야 한다.
//
// Rust에서는 이러한 입력을 std::env.args().collect()를 이용해서 받을 수 있다.
//
// 아래는 위에서 예를 든 2개 문자열을 인자로 받는 코드이다.
// 여기서 &args[1]과 같이, 인덱스가 0이 아니라 1부터 시작함에 유의한다.
//
// 인덱스 0은 실행한 프로그램의 이름이 된다.

use std::env;

// 구조체를 이용해서 커맨드 라인 처리하기
struct ArgsConfigs {
    src_dir: String,
    dst_dir: String,
}

// env::args().collect()로부터 ArgsConfig를 생성하는 build 메서드
impl ArgsConfigs {
    // (1) build의 파라미터에 self가 없다. 즉, 메서드가 아니라 함수다.
    // 따라서 ArgsConfig::build(...)와 같은 형태로 실행될 수 있다.
    // 그리고, build 함수의 파라미터로 &[String]타입의 값을 받게 되어 있다.
    // 즉, String을 담고 있는 배열 혹은 벡터를 파라미터로 받을 수 있는 것이고,
    // &가 있기에 객체의 주소를 받는 참조형이어서 소유권이 이동하지는 않는다.
    fn build(args: &[String]) -> Result<ArgsConfigs, &'static str> {
        // (2) 리턴 타입이 Result다. Result 열거형은 Ok와 Err을 베리언트로 갖는다.
        // Ok에 실제 리턴되는 값이 들어가게 되는데, 여기서는 ArgsConfig 구조체 형태의 값이 들어가게 되어 있다.
        // Err에 들어가는 타입은 문자열이다.
        if args.len() < 3 {
            // (3) args[0]은 프로그램 이름이되고,
            // args[1]이 src_dir 이름이, args[2]에는 tgt_dir 이름이 들어가게 된다.
            // 따라서, args에는 최소한 3개의 값이 있어야 한다. 만약 3보다 작은 크기면 에러를 리턴하게 한다.
            return Err("not enough arguments");
        }
        let src_dir = args[1].replace("\"", "");
        // (4) src_dir을 나타내는 문자열에는 더블 따옴표가 들어갈 수 있다.
        // 만약 그렇다면, 폴더 이름만을 사용해야하기에 더블 따옴표를 없애야한다. replace 메서드를 이용해서 없앨 수 있다.
        let dst_dir = args[2].replace("\"", "");

        // (5) Ok에 ArgsConfig 구조체를 담아서 리턴한다.
        Ok(ArgsConfigs { src_dir, dst_dir })
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let src_dir = &args[1];
    // let dst_dir = &args[2];

    // 프로그램을 실행하면서 추가로 입력하게 되는 입력값들을 std::env::args().collect();을 통해서 받을 수 있다.
    // 이를 통해 입력값들이 String 형태로 벡터에 들어가게 된다.
    let args: Vec<String> = env::args().collect();

    // build를 static 함수로 선언했기에(self를 파라미터로 사용하지 않음)
    // ArgsConfig::build(&args)와 같은 형태로 실행을 할 수 있다.
    let args_config = ArgsConfigs::build(&args).unwrap_or_else(|err| {
        // build의 리턴 타입이 Result이기에 Ok인 경우와 Err인 경우에 대한 처리를 해야한다.
        // 여기서는 unwrap_or_else 를 사용해서 처리한다. 이 메서드는 Ok의 경우는 그 안에 담고 있는 값을 리턴하고,
        // Err의 경우는 클로저를 실행한다. 따라서, 클로저에 Err의 경우에 수행할 코드를 적으면 된다.
        println!("Problem building args: {}", err);
        // (4) Err의 경우 에러 메시지를 출력하고 프로그램을 종료하게 했다.
        std::process::exit(1);
    });

    // (5) 앞 코드에서 커멘드 라인 입력으로 들어온 인자를 ArgsConfig 구조체에 담아 뒀기에,
    // 이 구조체의 값을 args_config.src_dir와 같은 형태로 읽으면 된다.
    let src_dir = args_config.src_dir;
    let dst_dir = args_config.dst_dir;

    println!("src_dir: {}, dst_dir: {}", src_dir, dst_dir);
}
// env::args().collect()를 Vec<String>으로 받아 처리했다. 대부분 이렇게 벡터 스트링으로 처리하면 된다.
// args[0]는 프로그램 자신의 이름이다. args[1] 부터가 인자를 받는 문자열이다.
