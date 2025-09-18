fn main() {
    //for_each도 꽤 많이 사용되는 Iterator Adapter
    // map처럼 원소 각각에 대해 뭔가 액션을 한다.
    // 다른 점은, map은 액션을 한 후 그 결과를 또 다른 Iterator로 만들어서 리턴하는 반면,
    // for_each는 액션을 그냥 수행해서 종결한다는 점'

    //1. 각 원소에 대해 +1을 해서 업데이트
    let mut v = vec![1, 2, 3];
    v.iter_mut().for_each(|x| *x += 1);
    println!("{:?}", v);

    //2. 홀수 인덱스에는 1, 짝수 인덱스에는 0을 가지는 배열
    let mut v = vec![1; 10];
    v.iter_mut()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 0)
        .for_each(|(_, val)| *val = 0);
    println!("{:?}", v)
}
