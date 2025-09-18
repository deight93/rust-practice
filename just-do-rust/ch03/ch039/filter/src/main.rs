fn main() {
    // filter는 괄호 안에 있는 클로저의 조건을 만족하는 원소들만으로 재 구성된 iterator를 리턴한다.

    /*
    let v1: Vec<_> = v.iter().filter(|x| **x % 2 == 0).collect();
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();

    # 두가지 *, ** 되는 이유

    v.iter() → Item = &i32
    filter의 프레디킷 인자 타입 → &Self::Item = &&i32
    클로저에서 |x| 라고 받으면 x: &&i32

    이제 두 버전:

    **x % 2 == 0

        x: &&i32 → *x: &i32 → **x: i32
        완전히 값 i32로 만든 뒤 % 연산 → 당연히 OK.

    *x % 2 == 0
        x: &&i32 → *x: &i32 (여기까지만 직접 deref)
        이제 좌변은 &i32인데, %는 내부적으로 std::ops::Rem 트레잇 호출입니다.
        연산자 트레잇 호출 시에도 메서드 해석과 동일하게 “autoderef”가 적용됩니다.
        즉, &i32를 i32로 자동으로 한 번 더 벗겨서(*를 암묵 삽입) i32 % i32로 맞춰줍니다.
        i32는 Copy라 값 복사로 문제도 없습니다.
        그래서 *x % 2도 컴파일이 됩니다.

    요약하면:
        **x는 “수동으로 두 겹 다 벗긴” 것이고,
        *x는 “한 겹만 벗기고, 나머지 한 겹은 연산자 호출 시 컴파일러가 자동으로 벗겨준” 것입니다.
     */
}

#[test]
fn filter_test1() {
    // 원소 중 짝수인 것만을 추려서 벡터로 만드는 코드

    // filter 안의 클로저 형태를 보면 &x로 받았다.
    // 전달되는 것이 레퍼런스 형태인 &i32이기에 &x : &i32가 되서
    // 결국 x : i32형태로 dereferencing이 되는 것이다. 이렇게 해야 x%2==0 수행이 가능

    // filter는 클로저에 대한 조건을 만족하는 원소들만으로 새로운 iterator를 만든다는 것에 유의하자.
    // 따라서, 벡터로 변환할려면 filter(...).collect();를 해야한다.
    // 그리고, 이 collect()가 만드는 컬렉션이 어떤 형태인지 나타내 줘야한다. let v1:Vec<_> =
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|&x| x % 2 == 0).collect();

    println!("{:?}", v);
    println!("{:?}", v1);

    // 클로저를 |&x|로 받았다. 이것을 |x|
    // 클로저를 |x|로 받으면, x : &i32 형태가 된다.
    // 따라서, 이 x에다가 % 2를 하려면, dereferencing을 해야한다.
    // reference 변수를 dereferencing하는 것은 *를 붙이면 된다. 따라서, |x| *x%2==0과 같이 하면 되는 것
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", v);
    println!("{:?}", v1);

    // into_iter().filter은?
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
    // v.into_iter()에 의해 v에 대한 소유권이 넘어간 것이기에, 이 코드 밑 쪽에서 다시 v로의 접근은 에러가 발생
    // println!("{:?}", v); // error!
    println!("{:?}", v1);

    /*
    표준 라이브러리 명세상 Iterator::filter는 프레디킷을 FnMut(&Self::Item) -> bool 로 받습니다.
    v.iter()의 Item은 &i32이므로, 프레디킷이 받는 실제 인자는 &Self::Item = &&i32 입니다.
    그래서 “이중 참조”가 생깁니다.

    let v = vec![1, 2, 3, 4, 5];
    // 1) 인자 타입: &&i32  → 값 쓰려면 **x
    let v1: Vec<_> = v.iter().filter(|x| **x % 2 == 0).collect();

    // 2) 인자 패턴: &x 로 한 겹 벗김 → x: &i32 → 값 쓰려면 *x
    let v1: Vec<_> = v.iter().filter(|&x| *x % 2 == 0).collect();

    // 3) 인자 패턴: &&x 로 두 겹 벗김 → x: i32 (가장 깔끔)
    let v1: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
     */
}

#[test]
fn filter_test() {
    // 1. 모든 원소 중 짝수인 원소만 추려서 벡터로 만듦
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|x| **x % 2 == 0).collect();
    println!("{:?}", v1);

    // into_iter().filter
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", v1);

    //2. 10이상이면서 홀수인 원소만 추려서 벡터로 만듦
    let v = vec![1, 2, 3, 4, 5, 10, 11, 12, 27];
    let v1: Vec<_> = v.iter().filter(|&&x| x >= 10 && x % 2 == 1).collect();
    println!("{:?}", v1);

    // **x>=10 && **x%2==1"와 같이 해도 됨. 더블 * 사용
    let v = vec![1, 2, 3, 4, 5, 10, 11, 12, 27];
    let v1: Vec<_> = v.iter().filter(|x| **x >= 10 && **x % 2 == 1).collect();
    println!("{:?}", v1);

    //into_iter().filter
    let v = vec![1, 2, 3, 4, 5, 10, 11, 12, 27];
    let v1: Vec<_> = v.into_iter().filter(|x| x >= &10 && x % 2 == 1).collect();
    println!("{:?}", v1);
}
