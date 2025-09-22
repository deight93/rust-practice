// Rust에서 가장 많이 사용되는 열거형은 'Option'과 'Result'다.
// Option은 어떤 값을 리턴할 때 "값이 없을 수도 있을 때"를 위해 None이라는 타입이 있고,
// Result는 어떤 값을 리턴할 때 "에러가 있는 경우"를 위해 Err이라는 타입이 준비되어 있다.

// 어떤 값을 리턴하는 함수를 만들다 보면, 리턴할 값이 없는 경우가 있다.
// 어떤 id를 주고 해당 id에 해당하는 이름을 리턴한다고 했을 때,
// 해당 id 자체가 없을 수 있다. 이런 경우 '없다'는 것을 어떻게든 처리해야한다.
// 이런 경우는 너무나 많기 때문에 Rust에서는 Option이라는 열거형을 제공하고 있고, Option::None 타입으로 리턴하면 된다

/*
tip

값이 없을 때 다른 언어에서는 주로 Null을 리턴한다.
혹은 리턴값이 양의 정수인 경우 -1을 리턴하게해서 값이 없음을 알아채게하기도 한다.
이처럼 "값이 없는 경우"에 대한 처리 방법은 Rust에서는 표준화해서 Option::None으로 하라는 것이다.
 */

// 열거형 Option의 구조
// Option은 Some(T)과 None 타입으로 정의되어 있다.

/*
    enum Option<T> {
        Some(T),
        None,
    }
*/

// <T>라고 사용된 것은 '제네릭(Generic)' 표현이다.
// T 자리에 어떤 데이터 타입도 들어올 수 있다는 의미다. i32도 올 수 있고 f64도 올 수 있다.
// Option에 사용된 베리언트(variant)인 Some과 None은,
// Option::Some Option::None처럼 사용할 수도 있고, 그냥 Some None으로 사용할 수도 있다.
// 너무 많이 사용되기에 Rust에서는 Some과 None 타입을 별도로 만들어 뒀기 때문이다.

use std::collections::HashMap;
fn main() {
    // HashMap에서 해당 Key에 매칭되어 있는 값을 가져오는 메서드는 get이고, 이 메서드는 결과를 Option 타입으로 리턴한다.
    // 즉, 값이 있으면 Some<T>에 담아서 리턴하고, 찾으려는 Key가 없으면 None 타입으로 리턴한다.
    let map = HashMap::from([("Jeff", 80), ("Alice", 100)]);
    let name = "Jeff";
    let point = map.get(name).unwrap();
    println!("{} {}", name, point);

    // let name2 = "Bob";
    // let point = map.get(name2).unwrap(); //panic
    // println!("{} {}", name2, point);

    // 즉, 위와 같이 코드를 짜면 안된다. None이 있을 수 있는 경우는
    // unwrap()를 사용하면 안되고 None에 대한 처리를 해줘야 한다.
    // 아래 코드가 None을 처리해준 코드

    let name = "Jeff";
    match map.get(name) {
        Some(point) => println!("{} {}", name, point),
        None => println!("{}", name),
    }
    // match는 타입체크를 할 수 있다. 따라서, Some타입의 경우 그리고 None타입의 경우를 나눠서 처리할 수 있다.

    let name2 = "Jeff";
    if let Some(point) = map.get(name2) {
        println!("{} {}", name, point);
    } else {
        println!("{}", name);
    }
    // 위 예제처럼 Some 타입인 경우와 None타입의 경우처럼 두 가지의 경우 수만 있을 경우는
    // if let을 사용해서도 코드를 짤 수 있다. if let도 타입을 체크해서 분기할 수 있다.

    /*
    tip

    if let 이라는 것은 if와 완전히 다른 표현식이다. if let 다음에는 "<타입>=<값>"
    형태가 와서 '값'이 해당 '타입'인지를 비교한다. 따라서, <값>의 위치에는 Option 값 처럼
    'Some'타입 혹은 'None'타입이 될 수 있는 것이 와야한다.
    if let val = 1234와 같이, <값>의 위치에 이미 결정된 값이 오면 안된다는 것.

    비슷한 표현식으로 while let이 있다. 이것도 마찬가지로 "<타입>=<값>"의 형태다.
    while let Some(val) = num {...} 형태로 쓰이고, num이 Some 타입인한 계속 수행하라는 것이 된다.
    */
}
