fn main() {
    // let num_str = "010-1234-5678";으로
    // 선언된 num_str에서 숫자 부분만 뽑아내서 Vec:<u32>형태의 벡터에 저장하고, 전체 숫자를 프린트하는 프로그램을 짜세요.

    // filter_map
    let num_str = "010-1234-5678";
    let v: Vec<u32> = num_str.chars().filter_map(|d| d.to_digit(10)).collect();
    // filter_map은 map에서의 결괏값이 Some 타입의 경우만 filtering하는 메서드다.
    // filter_map(|c| c.to_digit(10))과 같이 하면 Some 타입으로 리턴되는 것만 필터링된다.
    // 한 줄로 모든 것을 처리할 수 있다.
    println!("{:?}", v);

    // match이용
    let num_str = "010-1234-5678";
    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        match c.to_digit(10) {
            Some(n) => v.push(n),
            None => {}
        }
    }
    println!("{:?}", v);

    // c.to_digit(10)의 리턴값을 match를 통해 처리했고, Some의 경우만 벡터에 push했다.
    // None의 경우 아무것도 처리하지 않는다. 이처럼 아무것도 처리하지 않을 때는 ()를 사용하면 된다.

    // if let 이용
    let num_str = "010-1234-5678";
    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        if let Some(n) = c.to_digit(10) {
            v.push(n);
        }
    }
    // if let은 match와 달리 Some의 경우에 대해서만 처리해도 된다.
    // 여기서는 to_digit(10)에서 리턴된 것이 Some 타입의 경우마 벡터에 push하게 했고,
    // None에 대해서는 아예 처리를 안했다.
    println!("{:?}", v);

    /*
    tip

    위 문제에 대한 풀이 방법 3개를 소개했는데 모두 알아야할까? 하는 의문이 들 것이다.
    그냥, 제일 코드가 간단한 filter_map만 알면되는 거 아닌가?
    반복자 어댑터를 사용하면 코드가 간단해지는 장점이 있으나, 디버깅하기는 힘들다.
    만약 의도한 결과가 안 나오면, 어디에서 문제가 생겼는지 스텝을 밟아가며 찾아야한다.
    그러나, map같은 반복자 어댑터의 경우 해당값이 필요할 때에 수행되는
    Lazy evaluation으로 동작하기에 스텝마다의 값을 확인하기가 쉽지 않다.
    이런 경우는 match나 if let을 이용하는 for 루프를 써서 구현한 후 각 스텝별로 확인하는 수 밖에 없다.

    즉, 모든 풀이 방법으로 다 구현할 줄은 알아야한다는 말이다.
     */
}
