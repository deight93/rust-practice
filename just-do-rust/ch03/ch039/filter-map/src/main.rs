fn main() {
    // 어떤 문자열 벡터가 있는데, 문자열에 숫자도 있고 그냥 알파벳도 있고 한 상태에서,
    // 숫자 문자열에 대해서만 i32 형태의 숫자로 바꿔서 벡터로 저장하고자 한다고 하자.
    // "1"과 "5"만 숫자로 변형해서 [1, 5]와 같은 벡터를 뽑아내려는 것
    let a = ["1", "two", "NaN", "four", "5"];

    let v: Vec<_> = a
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();

    println!("{:?}", v);

    /*
    map - filter -map 형태를 띤다.

    첫 번째 map에서 문자열에 대해 parsing 시도를 하고,
    parsing 시도에서 ok된 것만을 filtering해서,
    그것들 만을 대상으로해서 unwrap()했다.

    이처럼 map안의 클로저에 대한 결과가 Option 형태이면,
    이 Option 형태에서 Some(value)의 value 부분만을 뽑아내는 것이 map_filter 메서드
     */

    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a.iter().filter_map(|s| s.parse::<i32>().ok()).collect();

    println!("{:?}", v);
    /*
    filter_map의 괄호 안에 있게되는 클로저의 리턴 타입은 Option 타입이어야 한다.
    위 코드에서 s.parse()의 리턴값은 Result이고,
    이를 Option 타입으로 바꿔주는 메서드가 ok() 메서드이다.
    따라서, filter_map안을 |s| s.parse::<i32>().ok()처럼 사용한 것
     */
}
