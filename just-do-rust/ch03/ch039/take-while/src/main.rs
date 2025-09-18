fn main() {
    // take_while은 filter와 유사하게 클로저의 조건을 만족하는 원소들만 추려내서 iterator를 만든다.
    // 다른 점은 filter는 조건을 모든 것을 원소 끝까지 뒤지는 반면,
    // take_while은 조건을 벗어나는 원소를 처음 만나면 더 이상 찾는것을 하지 않는다.

    let v = vec![1, 3, 5, 6, 7, 8, 9, 10];
    let v1: Vec<_> = v.iter().filter(|&&x| x % 2 == 1).collect();
    let v2: Vec<_> = v.iter().take_while(|&&x| x % 2 == 1).collect();
    // let v1: Vec<_> = v.iter().filter(|x| **x % 2 == 1).collect();
    // let v2: Vec<_> = v.iter().take_while(|x| **x % 2 == 1).collect();

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    // 위 코드에서 보면, filter의 경우는 벡터 v에 있는 모든 홀수를 골라낸다.
    // 반면, take_while은 홀수가 아닌 6을 만나는 순간 더 이상 진행을 하지 않는다
}
