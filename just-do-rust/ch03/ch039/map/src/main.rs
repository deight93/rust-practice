fn main() {
    // 반복자의 요소 하나하나에 대해 어떤 연산을 하게하는 것이다.
    // 일반적인 프로그래밍에서 for 루프에서 하는 작업과 비교해서 생각하면 이해가 될 것이다.

    // for_each와 유사한데, 다른 점은, for_each는 리턴값이 없이 종결하는 것이고,
    // map은 Map이라는 iterator를 다시 리턴한다는 점.

    // 어떤 벡터에 대해서 그 벡터의 값을 바꾸고자 한다면 v.iter_mut().for_each(...);를 사용해서 바꾸는 게 좋고,
    // 어떤 벡터의 값을 읽어서 가공 후 다른 벡터로 저장하길 원하면 v.iter().map(...).collect();와 같이 하는게 낫다.

    /*
    iter() vs. into_iter()

    그런 어떤 경우에 iter()를 쓰고, 또 어떤 경우에 into_iter()를 쓰는 걸까?

    iter(): iterator를 쓰고 난 후, 해당 컬렉션으로의 접근을 해야하는 경우 사용
    into_iter(): 해당 컬렉션을 다시 사용하지 않을거라고 생각되는 경우

    또한, (0..100).into_iter() 처럼 범위에 대한 iterator는 into_iter()만 가능하다. iter()는 안된다.
     */
}

#[test]
fn map_test1() {
    // 벡터에 있는 모든 값을 읽어서 1씩 더한 후, 그 결괏값을 다른 벡터로 저장
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().map(|&x| x + 1).collect();

    println!("{:?}", v);
    println!("{:?}", v1);

    /*
    map(...)에 의해서는 또 다른 iterator가 생성됨에 유의
    이것을 어떤 객체로 만들어서 마무리하려면, iterator에서 제공하는 메서드를 사용

    collect() : iterator의 내용을 collection으로 만든다. 어떤 컬렉션으로 만들지는 지정해줘야한다.
    sum() : iterator의 내용을 합한 결과를 리턴. 결과가 어떤 타입(u32 등)일 지는 지정해줘야 한다.
    max() : 최댓값을 리턴. 타입 지정 필요
    min() : 최솟값을 리턴. 타입 지정 필요
    count() : iterator의 원소 개수 리턴
    product(): iterator의 각 원소를 곱한 결과를 리턴. 타입 지정 필요

    클로저에 |&x|와 같이 사용했다. iter()를 사용했기에 클로저에 전달되는 것은 &i32 타입 원소가 전달
    &x로 받은 것이기에 de-referencing이 된다.
    즉, &x=&i32인 셈이기에 x=i32가 되어, 클로저 x에는 벡터의 값이 복사되어 전달된다.
    따라서, 뒤에 오는 x+1에서의 x는 복사된 x

    |x|라고 전달해도 문제없이 동작한다.
    |x|라고 사용하면 x=&i32가 되어 x는 참조자다.
    따라서, 뒤에 오는 x+1에서의 x는 참조자인데,
    Rust에서는 primitive에 대한 연산의 경우 참조자가 가리키는 값을 가지고 덧셈이 이루어지기에 결과는 똑 같은 것
     */
}

#[test]
fn map_test2() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().map(|x| x + 1).collect();

    // println!("{:?}", v);
    println!("{:?}", v1);

    /*
    v.into_iter()를 사용하고 난 후, 그 아래 쪽에서 println!("v={:?}",v); 한 부분에서 에러가 발생한다.
    v.into_iter()에 의해 벡터 v의 소유권이 넘어갔기 때문에, 다시 v로의 접근이 안되는 것
    into_iter()는 컬렉션의 원소 자체가 전달되는 메서드이기에 원소 자체가 i32 타입이다. 이를 &x와 같이 쓸 수 없기 때문
     */
}

#[test]
fn map_test3() {
    // 1~100까지의 수에 대해 제곱을 한 것을 벡터로 만드는 코드
    // 범위에 해당하는 (1..=100).map이라고 한 것에 유의. (1..=100) 자체가 일종의 iterator

    // map(...).collect()에 의해 컬렉션이 생성되는데,
    // 그 컬렉션이 어떤 타입이 될 것인지 let ans:Vec<_> =와 같이 Vec<_>를 한 것에 유의
    // ec 안에 있는 <_>은 iterator 안에 있는 데이터 타입에 따라 결정하겠다는 의미이다.
    // 명시적으로 Vec<u32>와 같이 지정해도 됨
    let ans: Vec<_> = (1..=100).map(|x| x * x).collect();
    println!("{:?}", ans);

    // let ans:Vec<_> =와 같이 변수에 어떤 컬렉션으로 collect()되는 지를 지정하지 않고
    // collect::<Vec<_>>();와 같이 collect에다가 어떤 컬렉션이 될 지 지정해준 것
    let ans: Vec<_> = (1..=100).map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", ans);
}

#[test]
fn map_test() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v1);

    let v2: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", v2);

    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().map(|x| x * 2).collect();
    println!("{:?}", v1);

    let v = vec![1, 2, 3, 4, 5];
    let v1 = v.iter().map(|x| (*x) * 2).collect::<Vec<u32>>();
    println!("{:?}", v1);

    //2. 문자열을 모두 소문자로 변환
    let words: Vec<&str> = vec!["hello", "Good Morning", "Hi"];
    let low_words: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
    println!("{:?}", low_words);

    //3. 문자열에서, 각 문자에 대해 문자열에서 해당 문자의 개수를 HashMap으로 저장. (ch: cnt)
    use std::collections::HashMap;
    let s = "abc aaa bb c";
    let map: HashMap<char, usize> = "abc"
        .chars()
        .map(|c| (c, s.matches(c).count()))
        .collect::<HashMap<char, usize>>();
    println!("{:?}", map);

    // (참조)어떤 문자열에서 알파벳만을 집합으로 뽑아낼 때
    use std::collections::HashSet;
    let s = "abc aaa bb c";
    let set = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<_>>();
    println!("{:?}", set);
}
