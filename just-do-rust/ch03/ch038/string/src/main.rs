fn main() {
    //String은 문자열을 다루는 대표적인 문자열 타입니다.
    // &str은 고정된 문자열을 다루는 타입인데 반해, String은 가변 크기 문자열을 다룬다.

    // &str 타입의 변수는, 프로그램의 바이너리에 있었던 문자열이 메모리로 옮겨지면
    // 그 메모리 주소를 가리키는 레퍼런스 변수가 된다.
    // 반면, String 타입 변수는 힙에 저장되는 문자열 값에 대한 소유권을 가진 변수가된다.
    // 물론 다른 변수에게 &String 형태로 대여할 수도 있다.

    // 1.String 변수의 생성
    let hello = String::from("Hello, world!");
    println!("{}", hello);
    // 힙 메모리를 사용하는 다른 객체인 벡터, HashMap과 유사하게, String 변수도 from 메서드를 이용해서 만들 수 있다.

    let s1 = "Hello, ".to_owned();
    let s2 = "world!".to_string();
    println!("{}{}", s1, s2);

    //2. 문자열 합치기
    // String의 push_str 이용
    let mut s1 = String::from("hello, ");
    let s2 = "world!";
    s1.push_str(s2);
    assert_eq!("hello, world!", s1);

    // + 오퍼레이션 이용
    // String 변수 + &String 변수 혹은 String 변수 + &str 변수 형태가 된다.
    let a = String::from("hello, ");
    let b = String::from("world!");
    let c = a + &b;
    assert_eq!("hello, world!", c);
    println!("{}", c);
    // println!("{}", a); //error 소유권 이동되었음
    println!("{}", b); // b는 대여를 해서 사용하는 형태, let c = a + &b; 코드 이후에도 변수 b로의 접근은 문제 없다.

    // String + &str
    let s1 = String::from("hello, ");
    let s2 = "world!";
    let s3 = s1 + s2;
    assert_eq!("hello, world!", s3);

    //format! 매크로 이용
    let a = String::from("hello, ");
    let b = String::from("world!");
    // &String 이용
    let c = format!("{}{}", &a, &b);
    println!("c={}", c);
    // &str 이용
    let c = format!("{}{}", a, b);
    println!("c={}", c);
    // String 이용
    let c = format!("{}{}", "hello, ", "world!");
    println!("c={}", c);

    //3. String의 메서드
    // as_bytes
    let s = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    //as_str
    let s = String::from("hello");
    assert_eq!("hello", s.as_str());

    //clear
    let mut s = String::from("hello");
    s.clear();
    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(5, s.capacity());
    assert_eq!("", s);

    //from_utf8
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];
    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);

    //insert
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');
    assert_eq!("foo", s);

    //insert_str
    let mut s = String::with_capacity(3);
    s.insert_str(0, "foo");
    assert_eq!("foo", s);

    //into_bytes
    let s = String::from("hello");
    let bytes = s.into_bytes();
    assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);

    //push
    let mut s = String::from("lo");
    s.push('l');
    assert_eq!("lol", s);

    //remove
    let mut s = String::from("abc");
    assert_eq!(s.remove(0), 'a');
    assert_eq!(s.remove(1), 'c');
    assert_eq!(s.remove(0), 'b');

    //truncate
    let mut s = String::from("hello");
    s.truncate(3);
    assert_eq!("hel", s);
}
