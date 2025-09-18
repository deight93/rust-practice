fn main() {
    // 반복자의 생성은 데이터의 '모음' 혹은 수의 '범위'에서 이루어진다.
    // 데이터의 '모음'은 Rust에서 Collection이라고 부르는 것들이고 벡터(Vec), 배열, 해시 맵 같은 것들이다.
    // 수의 범위는 1..100와 같이 표현되는 범위를 말한다.
    // '데이터 모음'이나 '수의 범위'에서 iterator(반복자)를 뽑아내는 메서드는
    // iter(), into_iter(), iter_mut()가 있다.

    /*
    iter() : 반복자에 의해 접근되는 원소들의 레펀런스가 넘어온다. 소유권이 이동되는 것이 아니다.
    따라서, iter()에 의한 반복자로 원소들을 사용하고 난 후에도, 해당 컬렉션에 대한 사용이 가능하다.
    컬렉션의 소유권이 이동하지 않았기 때문이다.

    into_iter() : 컬렉션 자체가 넘겨져서, 소유권이 넘어가 버리기에,
    into_iter()를 수행하고 난 후에는, 해당 컬렉션 변수로의 접근이 안된다.

    iter_mut(): 컬렉션의 값을 수정해야할 때 사용한다. 레퍼런스로 받은 다음에 수정하는 것이다. 소유권이 넘어가지는 않는다.
     */
}

#[test]
fn iter_test() {
    // 1. iter()
    // 벡터에 대해 iter()를 통해 반복자를 뽑아내서 사용하는 예

    // v.iter()를 이용해서 반복자를 만들어서, for 루프에서 사용하면 된다.
    // 이 때 벡터에 대한 접근은 reference로 하기 때문에 소유권이 넘어가지 않는다.

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    for val in v.iter() {
        println!("{}", val);
    }

    println!("======================");

    for val in &v {
        println!("{}", val);
    }
}

#[test]
fn into_iter_test() {
    // 2. into_iter()
    // v.into_iter()에 의해 벡터 v에 대한 소유권이 넘어갔기에,
    // 다시 v에 대한 접근을 하는 for in &v에서 에러가 발생하는 것

    // 이처럼, iterator 사용 후에 다시 해당 컬렉션을 접근해야하는 경우는 iter()를 사용하는 게 편리하고,
    // 그렇지 않은 경우는 into_iter()를 사용한다.

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    for val in v.into_iter() {
        println!("{}", val);
    }

    println!("======================");

    // for val in &v {
    //     //여기서 에러 발생한다. 위에서 v.into_iter()에 의해 소유권이 넘어갔기 때문.
    //     println!("{}", val);
    // }
}

#[test]
fn iter_mut_test() {
    // 3. iter_mut()
    // iter()와 into_iter()를 이용해서는 컬렉션의 값을 바꿀 수 없다.
    // 컬렉션의 값을 바꿀려면 iter_mut() iterator를 사용해야 한다.
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    for val in v.iter_mut() {
        *val += 1;
    }
    println!("{:?}", v); // [2,3,4,5,6]

    println!("======================");

    for val in &v {
        println!("{}", val);
    }
    //  v의 iterator가 v.iter_mut()로 얻어졌고,
    // 이를 이용해서 벡터에 있는 각 원소에 1 씩 더해졌다.
    // 또한, v.iter_mut()는 iter() 처럼, iterator를 얻어내고 난 다음에도
    // 해당 벡터에 대한 접근을 다시 할 수 있다. 즉, 소유권이 넘어가거나 하지 않는다.

    // iter_mut()에 대해 for 루프를 사용해서 값을 바꿨는데,
    // iterator에서 제공하는 for_each메서드를 이용하면 좀 더 쉽게 구현 가능
}

#[test]
fn iter_mut_for_each_test() {
    // for_each는 iterator에서 제공하는 메서드 중 하나로,
    // for_each 다음에 있는 괄호 안에 있는 클로저를 반복해서 실행한다.

    /*
    클로저는 함수형 언어의 한 특징으로, 일종의 익명함수다.
    익명함수라고 하는 이유는 함수의 이름이 없으면서 함수와 같은 역할을 하기 때문.
    위 for_each 메서드의 괄호 안에 있는 것이 일종의 클로저인데, 파라미터를 |x|처럼 | |로 받아서 처리한다.
    즉, val를 파라미터로 받아서 *val += 1이라는 작업을 마치 함수처럼 동작시킨다.
     */

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    v.iter_mut().for_each(|val| *val += 1);
    println!("{:?}", v);

    for val in &v {
        println!("{}", val);
    }
}
