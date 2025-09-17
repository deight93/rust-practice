use std::collections::HashMap;

fn main() {
    // 1. HashMap의 선언과 생성
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("hello", 1);
    map.insert("world", 2);
    println!("{:?}", map);

    let map = HashMap::from([("hello", 1), ("world", 2)]);
    println!("{:?}", map);

    // 2. 벡터로부터 HashMap 생성하기
    // zip 메서드는 반복자 2개에 대해서 1:1로 데이터를 짝짓기 해준다.
    // 여기서는 name에 대해서 3개의 원소가 있고 score에 대해서 3개의 원소가 있으니, 하나씩 짝을 지어서 리턴해준다.
    // zip에 의해서 ("hello",1)과 같이 짝이 지어진 3개의 튜플이 생기고,
    // 이것이 collect에 의해서 HashMap으로 생성되는 것이다.
    let name = vec!["hello", "world"];
    let score = vec![1, 2];
    let map: HashMap<_, _> = name.iter().zip(score.iter()).collect();
    println!("{:?}", map);
    /*
    위에서 HashMap<_,_>과 같이 자동 추론을 사용했다.
    이 경우는 HashMap<&str,i32>과 같이 타입을 명시하는 것도 어렵지 않다.
    그러나 여기서 사용한 into_iter()가 아닌 iter() 반복자를 사용한 경우는 복잡해진다.
    iter()는 참조자를 리턴하기 때문이다.
    name.iter().zip(score.iter()).collect();라고 한 경우, 이 리턴 타입은 (&&str, &i32)이다.
    따라서, 이것을 명시적 타입지정을 해서 사용하려면
    let map:HashMap<&&str,&i32> = name.iter().zip(score.iter()).collect();와 같이 해야한다.

    즉, collect()에 의해 어느만큼 &를 붙여야할지 헷갈린다. 따라서 HashMap<_,_>와 같이 처리하는 것이 속편하다.
     */

    // 3. HashMap 데이터 접근하기
    let map = HashMap::from([("hello", 1), ("world", 2)]);

    // Key를 지정해서 get
    println!("{}", map.get("hello").unwrap());

    // HashMap에 있는 모든 Key-Value
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Key를 얻어낸 후 Value 액세스
    for k in map.keys() {
        if k.starts_with("w") {
            print!("{} ", map.get(k).unwrap())
        }
    }

    // 4. 데이터 갱신 하기
    let mut map = HashMap::from([("hello", 1), ("world", 2)]);

    map.insert("hello", 3);
    println!("{:?}", map);
    // 출력한 것을 보면 순서가 보장이 안된다.
    // 즉, 입력한 순서로 출력되지 않는다. HashMap의 특징이다. (벡터는 순서가 보장된다. )

    // 만약 "HashMap에 이미 Key가 있다면 갱신하지 않고, Key가 없는 것에 대해서만 값을 쓰자"고 했을 때, 어떻게 하면 될까?
    let mut map = HashMap::from([("hello", 1), ("world", 2)]);

    let new_data = [("hello", 3), ("rust", 4)];
    for (key, value) in &new_data {
        map.entry(key).or_insert(*value);
    }
    println!("{:?}", map);
    // map.entry(k).or_insert(*v);에서 *v라고 한 것은
    // for (k,v) in &new_data{에 의해 얻어진 v가 참조값이기에 de-referencing 한 것이다.

    // 위에서 사용한 entyr(key).or_insert(val)은 &val을 리턴한다.
    // 즉, "val값이 저장되어 있는 주소"를 리턴한다.
    // 이 성질을 이용하면, 어떤 문장에서 문자가 혹은 단어가 몇 번 나왔는지를 셀 때 유용하게 쓸 수 있다.

    // 문자를 세는 코드
    let text = "stay foolish stay hungry";
    let mut map = HashMap::new();
    for c in text.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // 단어를 세는 코드
    let text = "stay foolish stay hungry";
    let mut map = HashMap::new();
    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // 5. 해시맵의 메서드

    // clear()
    let mut a = HashMap::new();
    a.insert("hello", 1);
    a.insert("world", 2);
    println!("{:?}", a);
    a.clear();
    assert!(a.is_empty());

    //contains_key()
    let mut map = HashMap::new();
    map.insert(1, "aa");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);

    //get
    let mut map = HashMap::new();
    map.insert(1, "aa");
    assert_eq!(map.get(&1), Some(&"aa"));
    assert_eq!(map.get(&2), None);

    //insert
    // 주어진 key-value 페어를 HashMap에 집어 넣는다.
    // 만약 HashMap의 기존 Key에 집어 넣으려는 key가 없다면, key-value를 추가하고 `None`을 리턴한다.
    // 그런데, 만약 기존 Key에 집어 넣으려는 key가 이미 존재한다면,
    // 해당 value를 신규 값으로 갱신하고, 예전 값을 `Some`에 담아서 리턴한다.
    let mut map = HashMap::new();
    assert_eq!(map.insert(2, "cc"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(1, "bb");
    assert_eq!(map.insert(1, "dd"), Some("bb"));
    assert_eq!(map[&1], "dd");

    //keys
    // 해시맵이 가지고 있는 모든 Key에 대한 반복자를 리턴한다.
    // 이 때 Key들의 순서는 정렬되어 있지도 않고 입력된 순서를 보장하지 않는다.
    let mut map = HashMap::from([("hello", 1), ("world", 2)]);
    for k in map.keys() {
        println!("{}", k);
    }

    //remove
    let mut map = HashMap::from([("hello", 1), ("world", 2)]);
    map.insert("rust", 3);
    assert_eq!(map.remove("hello"), Some(1));
    assert_eq!(map.remove("hello"), None);

    //values
    let mut map = HashMap::from([("hello", 1), ("world", 2)]);
    for v in map.values() {
        println!("value = {v}");
    }
}
