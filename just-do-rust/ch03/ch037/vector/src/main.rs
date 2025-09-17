fn main() {
    //1. 벡터의 생성/선언 방법
    // 벡터를 선언/생성할 때 두 가지 방법이 있다.
    //
    // Vec::new();
    // vec! 매크로 이용

    // Vec::new()
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    // vec! 매크로
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    let v = vec![5; 5];
    println!("{:?}", v);

    // arr.to_vec();
    let arr = [1, 2, 3];
    let mut v = arr.to_vec();
    v.push(5);
    println!("{:?}", v);

    // extend();
    let mut v = vec![1, 2, 3];
    v.extend([4, 5, 6]);
    println!("{:?}", v);

    //vector 에서 index 접근
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    println!("last element: {}", v[v.len() - 1]);

    //get()
    let v = vec![1, 2, 3];
    //println!("{}", v[3]); // panic 발생
    println!("{:?}", v.get(3));
    if let Some(n) = v.get(1) {
        println!("v.get(1)={}", n);
    }

    // 3. 벡터의 원소에 접근하기 (반복자 이용)
    /*
    벡터는 IntoIterator 트레이트를 구현해놨다.
    따라서, v.into_iter() 혹은 v.iter() 대신에 for a in v 혹은 for a in &v라고 해도 된다. 동일하다.

    * for a in v.into_iter()  == for a in v
    * for a in v.iter() == for a in &v
    // for a in &v를 하면 벡터 v에 있는 반복자가 레퍼런스 형태로 나온다.
    // for a in v를 하면 벡터 v에 있는 반복자가 moved형태로 나온다.
    // 즉, 소유권까지 가지고 나온다. 따라서, 이렇게 사용한 경우, 이후 코드에서 v를 다시 사용할 수 없다.
     */

    //레퍼런스(참조)로 접근하기
    // for a in &v에 의해 원소에 접근할 때는, a는 원소의 레퍼런스이기에 *a를 해야 원소의 내용에 접근하는 것이다.
    // 그러나, println!을 하는 경우는 a로 접근하는거나 *a로 접근하는 거나 동일하게 처리한다.
    // println! 매크로에서 두 경우 모두를 처리할 수 있게 구현되어 있기 때문이다.
    let v = vec![1, 2, 3, 4, 5];
    for a in &v {
        println!("{}, {}", a, *a);
    }

    for a in v {
        println!("{}", a);
    }
    // println!("{:?}", v); // error

    // 4. vector의 element 업데이트 하기
    // 4.1 인덱스로 접근해서 값 업데이트
    // 0부터 v.len()까지로 인덱스 범위를 주고, 해당 인덱스에 해당하는 v[i]의 값을 업데이트 하는 거다. 직관적이고 쉽다.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    for i in 0..v.len() {
        v[i] = i + 2;
    }
    println!("{:?}", v);

    // 4.2 mutable 반복자 이용 &mut v
    // &mut v라고 하면 해당 벡터의 mutable한 반복자가 나온다.
    // 참조자(레퍼런스)로 나오기 때문에, 해당 원소의 값에 접근하려면 *a라고 해줘야 한다.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    for a in &mut v {
        *a += 10;
    }
    println!("{:?}", v);

    // 4.3 mutable 반복자 이용 : v.iter_mut()
    let mut v = vec![1, 2, 3, 4, 5, 6];
    for a in v.iter_mut() {
        *a += 5;
    }
    println!("{:?}", v);

    // 4.4 iterator adapter 이용
    // Iterator Adapter는 Iterator의 모든 원소에 대해 계산한 결과를 다시 Iterator로 토해내는 패턴을 얘기한다.
    // 여기서는 iter_mut().for_each()라고 해서 Iterator Adapter라고까지 해야하나 싶지만,
    // 여튼 반복자 다음에 .method() 형태라서 Iterator Adapter라고 한거다.
    // for_each 메서드는 반복자 안에 있는 모든 원소에 대해 어떤 작업을 한다.
    // 위 예제 1~3번에 비해 코드가 깔끔하다.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.iter_mut().for_each(|a| *a += 20);
    println!("{:?}", v);

    // 5. 벡터를 스택으로 사용하기
    // Rust에서는 따로 스택(Stack)에 해당하는 데이터 구조를 제공하지 않는다. 그냥 벡터를 사용하면 된다.
    //스택은 아마도 가장 많이 사용되는 데이터 구조일 것이다. LIFO(후입선출, Last In First Out) 구조다.
    //접시를 쌓아 올리는 구조로 생각하면된다. 접시를 넣을 때는 기존 접시 위에 쌓아지는 형태이고,
    //접시를 뺄 때는 제일 위에 있는 접시를 빼는 거다. 이는 제일 마지막에 올려놓은 접시다. 해서 후입 선출

    // 스택 선언: 그냥 벡터 선언을 하면 된다. let mut s:Vec<i32> = Vec::new();
    // push : 벡터의 push이용 s.push(0);
    // pop : 벡터의 pop()이용. 다만, pop()의 리턴 타입이 Option이기에 unwrap()해줘야 함

    let mut s: Vec<i32> = Vec::new();
    s.push(0);
    s.push(1);
    s.push(2);
    s.push(3);

    while s.len() > 0 {
        let i = s.pop().unwrap();
        println!("{}", i);
    }
    // 스택과 대비되는 데이터 구조는 '큐(Queue)'다. 스택과 다르게 FIFO(선입선출, First In First Out)이다.
    // 먼저 들어온, 오래된 데이터부터 나가는 구조다.
    // Rust에서 큐 데이터구조를 위해서는 VecDeque 혹은 LinkedList를 쓰면 된다.

    // 6. 벡터의 메서드
    // append()
    let mut v = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    v.append(&mut v2);
    println!("{:?}", v);

    // clear
    let mut v = vec![1, 2, 3];
    v.clear();
    assert!(v.is_empty());

    //len
    let v = vec![1, 2, 3];
    assert_eq!(3, v.len());

    // is_empty
    let mut v = Vec::new();
    assert!(v.is_empty());

    v.push(1);
    assert!(!v.is_empty());

    // insert()
    let mut v = vec![1, 2, 3];
    v.insert(1, 4);
    assert_eq!(v, vec![1, 4, 2, 3]);
    v.insert(4, 5);
    assert_eq!(v, vec![1, 4, 2, 3, 5]);
    println!("{:?}", v);

    // pop()
    let mut v = vec![1, 2, 3];
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v, vec![1, 2]);

    // push()
    let mut v = vec![1, 2];
    v.push(3);
    assert_eq!(v, vec![1, 2, 3]);
    println!("{:?}", v);

    // resize()
    let mut v = vec!["hello"];
    v.resize(3, "world");
    assert_eq!(v, vec!["hello", "world", "world"]);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4];
    v.resize(2, 0);
    assert_eq!(v, vec![1, 2]);
    println!("{:?}", v);

    // truncate()
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(3);
    assert_eq!(v, vec![1, 2, 3]);
    println!("{:?}", v);

    //fill
    let mut v = vec![0; 5];
    assert_eq!(v, vec![0, 0, 0, 0, 0]);
    v.fill(1);
    assert_eq!(v, vec![1, 1, 1, 1, 1]);

    //reverse
    let mut v = vec![1, 2, 3, 4, 5];
    v.reverse();
    assert_eq!(v, vec![5, 4, 3, 2, 1]);

    //sort
    let mut v = vec![5, 3, 1, 2, 4];
    v.sort();
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}
