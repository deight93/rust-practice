use std::collections::HashSet;

fn main() {
    // 1. 해시셋의 선언과 생성
    // 해시셋의 생성은 HashSet::new()를 통해 가능하다.
    // 값을 넣는 것은 insert 메서드를 사용하면 된다. 이미 있는 데이터는 덮어쓴다. 즉, 데이터의 중복을 허용하지 않는다.
    // 출력할 때는 {:?}를 사용하면 되는데, 순서가 보장되진 않는다.
    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set.insert(6);

    println!("{:?}", set);

    // 2. 배열/벡터로부터 해시셋 만들기
    // 배열로 생성
    let set = HashSet::from([1, 2, 3, 4, 5, 6]);
    println!("{:?}", set);

    // 추가
    let mut set = HashSet::from([1, 2, 3, 4, 5, 6]);
    set.extend([7, 8, 9]);
    println!("{:?}", set);

    //출력을 소팅
    let mut set = HashSet::from([1, 2, 3, 4, 5]);
    let mut v: Vec<i32> = set.into_iter().collect();
    v.sort();
    println!("{:?}", v);

    // 3. 해시셋 데이터로의 접근
    // 반복자를 iter() 혹은 into_iter()로 만들어서 HashSet 데이터에의 접근을 할 수 있다.
    // 아래 코드는 해시셋 데이터를 반복자로 꺼내서, 짝수의 경우만 다른 벡터에 집어넣는 코드다.
    let set = HashSet::from([1, 2, 3, 4, 5, 6]);
    let mut v: Vec<i32> = Vec::new();

    for x in set.iter() {
        if *x % 2 == 0 {
            v.push(*x);
        }
    }
    println!("{:?}", v);

    // 위 코드는 반복자 어댑터(Iterator Adapter)를 써서 더 간단히 짤 수 있다.
    let set = HashSet::from([1, 2, 3, 4, 5, 6]);
    let mut v: Vec<_> = set.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", v);

    // 4. 집합 연산
    let a = HashSet::from([1, 2, 3, 4, 5, 6]);
    let b = HashSet::from([3, 4, 5, 6, 7, 8]);

    let u: Vec<_> = a.union(&b).collect(); // 합집합
    let i: Vec<_> = a.intersection(&b).collect(); // 교집합
    let d: Vec<_> = a.difference(&b).collect(); // 차집합
    let s: Vec<_> = a.symmetric_difference(&b).collect(); // 대칭

    println!("{:?}", u);
    println!("{:?}", i);
    println!("{:?}", d);
    println!("{:?}", s);
}
