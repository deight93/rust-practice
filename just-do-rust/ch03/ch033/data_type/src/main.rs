fn main() {
    let a: u32 = 1; //타입을 u32로 명시했다.
    let b = 1; //타입을 명시하진 않았지만, 정수형인것을 알 수 있고, 이 경우 Rust는 i32타입으로 지정
    let c = "abc"; // &str 타입이 된다.
    // let d = "2".parse().expect("not a number");  //에러 발생한다.
    let d: u32 = "2".parse().expect("not a number"); // let d:u32 와 같이 변수 타입 결정해줌
    let d = "2".parse::<u32>().expect("not a number"); // parse::<u32>() 와 같이 parse 메서드의 리턴타입을 결정해줌

    /*
    Rust의 데이터 타입 종류

    스칼라 타입: 정수, 부동 소숫점, 문자, 부울
    복합 타입: 튜플, 배열
     */
}
