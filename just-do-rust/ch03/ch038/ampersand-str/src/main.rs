fn main() {
    // 우리가 일반적으로 선언하는 더블 따옴표로된 문자열은 &str 타입 변수로 받게 된다.
    // let s = "Hello, World";
    // s는 &str 타입 변수가 된다.
    // &str은 두 가지 정보를 가지고 있다. 하나는 일련의 바이트 값의 위치를 가리키는 포인터, 다른 하나는 그 길이.
    // 즉, 어떤 문자를 나타내는 바이트 값들의 위치와 그 길이다.

    // &str의 길이
    // Rust에서의 문자열은 항상 utf-8 인코딩을 사용한다.
    let s = "Hello";
    println!("{}", s.len());
    let s = "대한민국";
    println!("{}", s.len());

    // 문자 개수
    let s = "Hello";
    println!("{}", s.chars().count());
    let s = "대한민국";
    println!("{}", s.chars().count());

    // 문자열 슬라이스
    // Rust는 문자열에서 인덱스에 의한 접근을 허용하지 않는다. s[0]과 같은 접근은 안된다. &s[0]도 안된다.
    // "the type str cannot be indexed"라는 컴파일 에러가 난다.

    let s = "hello";

    // println!("{}", s[0]); //에러
    // println!("{}", &s[0]);  //에러
    // 이는, utf-8 방식으로 인코딩 되어 있어, 하나의 문자의 바이트 단위가 1이 아닌 1~4 바이트이기에,
    // 단일 인덱스로의 접근을 아예 막은 것이라 한다. 대신 슬라이스 범위로의 접근은 가능하다.

    println!("{}", &s[0..1]);
    // println!("{}", s[0..1]) // error
    // s[0..1] → 타입은 &str (슬라이스).
    // println!("{}", …)는 Display가 구현된 &str 참조를 원함.
    // 따라서 &s[0..1]처럼 한 번 더 참조를 넘겨야 동작.

    let s = "대한민국";
    // println!("{}", &s[0..1]); // 런타임 에러 발생
    // 하나의 바이트 만으로는 문자를 표현할 수 없다는 거다. 3바이트 범위를 해야만 한글 문자를 표현할 수 있다.
    println!("{}", &s[0..3]);

    // 문자열을 문자로 변환하기
    let s = "대한민국";
    println!("단어수={}", s.chars().count());

    let s = "010-9999-1234";
    println!("{:?}", get_number(s));
    println!("{}", s); //010-9999-1234

    // &str의 메서드들
    // byte
    let mut bytes = "bors".bytes();
    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());
    assert_eq!(None, bytes.next());

    // contains
    let bananas = "bananas";
    assert!(bananas.contains("na"));
    assert!(!bananas.contains("z"));

    //find
    // 주어진 pat 문자 혹은 문자열과 일치하는 문자열 슬라이스의 처음 위치 인덱스를 리턴한다.
    let s = "012345ABC";
    assert_eq!(s.find("0"), Some(0));
    assert_eq!(s.find("ABC"), Some(6));
    assert_eq!(s.find("Z"), None);

    //lines
    // 문자열을 라인 단위로 짤라서 문자열 슬라이스 반복자로 리턴한다.
    // 라인의 구분은 '\n' 혹은 '\r\n' 'r'만 있는 경우는 라인으로 구분하지 않는다.
    let text = "foo\r\nbar\n\nbaz\r";
    let mut lines = text.lines();

    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz\r"), lines.next());
    assert_eq!(None, lines.next());

    //parse
    // 문자열을 `u32` 등 다른 타입으로 변환한다.
    // Basic usage: 변환 타입을 변수에 명시
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);

    // 'turbofish' 방법: parse 메서드에 변환타입을 알려준다.
    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);

    //split_whitespace
    // 문자열에서 공백을 기준으로 문자열 슬라이스를 만들어내서 반복자로 리턴한다.
    let mut iter = "hello world".split_whitespace();
    assert_eq!(Some("hello"), iter.next());
    assert_eq!(Some("world"), iter.next());
    assert_eq!(None, iter.next());

    //trim
    // 문자열의 제일 앞과 뒤에 있는 공백을 제거한 후 새로운 str을 만들어서 &str 타입으로 리턴한다.
    let s = "\n Hello\tworld\t\n";
    assert_eq!("Hello\tworld", s.trim());

    // to_lowercase
    // 소문자로 바꿔서 String 타입으로 리턴한다
    let s = "HELLO";
    assert_eq!("hello", s.to_lowercase());
}

fn get_number(s: &str) -> Vec<u32> {
    // get_number 함수는 &str 타입 문자열을 chars()를 통해 문자들로 바꾸고, 이 문자들을 u32 타입 문자로 바꾼다.
    // c.to_digit(10)은 10진수 숫자로 바꾸는 메서드인데, 리턴 타입이 Option이다.
    // Option 타입 중 Some에 대해서만 처리하기 위해서는 filter_map 메서드를 쓰면 편하다.

    // 반복자에서 컬렉션을 만들 때 collect() 메서드를 쓰게 되는데, 이 때 컬렉션의 정확한 타입을 지정해줘야 한다.
    // 두 가지 방법이 있다. 하나는 let v:Vec<u32> = ......collect();와 같이 하는게 있고,
    // 다른 방법은 위 코드처럼 collect()메서드에 타입을 지정해주는 것. .collect::<Vec<u32>>()

    // main 함수에서 get_number(s)라고 한 후, 다시 s를 프린트 할 수 있다.
    // get_number(s)를 할 때 s의 타입 자체가 &str로 레퍼런스 타입이기에 소유권 이동이 발생하지 않는다.
    // 즉, get_number(s)라고 해도 변수 s에 대한 소유권 이동이 발생하지 않는다.
    s.chars()
        .into_iter()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
}
