// 함수의 파라미터와 리턴에 제네릭 타입을 사용할 수 있다.

// 두 값을 입력으로 받아서 그 중에서 작은 값을 리턴하는 함수를 제네릭으로 만들어 보겠다.
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
    // 당연한 얘기겠지만, 이 min 함수에 대해 i64 f32등 다른 타입의 값을 파라미터로해서 호출하면 에러가 난다.
    // 에러를 없애려면, 각 타입별로 동작하는 min 함수를 만들면 되겠지만, 제네릭 타입을 적용하면 한 개 함수로 처리가 가능
}

/*
// fn min_generic<T>(a: T, b: T) -> T {
//     if a < b { b } else { a }
    /*
    error[E0369]: binary operation `<` cannot be applied to type `T`
      --> just-do-rust/ch04/ch045/generic-function/src/main.rs:11:10
       |
    11 |     if a < b { b } else { a }
       |        - ^ - T
       |        |
       |        T
       |
    help: consider restricting type parameter `T` with trait `PartialOrd`
       |
    10 | fn min_generic<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
       |                 ++++++++++++++++++++++
     */
    // 위 코드에 대해서 컴파일 에러가 발생한다. 에러 내용 및 수정방법이 표시된다.

    // 이 에러가 발생한 것은 < 연산을 T 타입에서는 지원하지 않는다는 것이고,
    // 이를 해결하려면 T 타입에다가 무언가 제한을 해야한다는 거다.

    // 제네릭은 모든 타입을 지원하는 것이다. 어떠한 타입도 수용하겠다는 것이다.
    // 그런데 그러한 '모든' 타입의 두 값에 대해 if a < b로 값을 비교하려 했다.
    // 이 두 값이 정수형 타입(i16, i32 등)이거나 부동소숫점 타입(f32, f64 등)이면 < 연산이 가능하겠으나,
    // bool 타입이나 String 타입이라면 < 같은 비교 연산은 동작하지 않을 것이다.

    // 이러한 경우 제네릭 타입에 대해 한계를 지정해야한다. 즉, 사용 가능한 타입만을 지정해줘야 한다.
    // 이러한 것을 '트레잇 바운드'라고 하고, <T: std::cmp::PartialOrd>처럼 어떤 트레잇 이름을 명시해서,
    // 해당 트레잇을 구현한 타입만을 허용한다고 지정한다.

    // 위 경우는 PartialOrd 트레잇을 구현한 타입만 제네릭 타입으로 인정하겠다는 거다.
    // PartialOrd 트레잇은 비교 연산이 선언되어 있고, Rust의 모든 숫자형 타입은 이 트레잇이 구현되어 있다.
    // 따라서, i32 i64 f32 등 숫자형 타입에 대해서는 min 함수를 사용할 수 있는 것이다.
// }
*/

fn min_generic<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b { b } else { a }
}

/*
// 함수에 사용되는 제네릭 타입에 대해, 좀 더 복잡한 예제를 가지고 설명
// i32 타입의 두 수 a, b에 대해서 (a+b, a-b)를 튜플 형태로 리턴하는 함수
    fn add_sub(a: i32, b: i32) -> (i32, i32) {
        (a + b, a - b)
    }
// 이를 덧셈과 뺄셈이 지원되는 숫자형 타입에 대해서는 어떠한 타입도 가능하도록, 제네릭 타입 함수로 바꾸시오.


// add_sub 함수를 제네릭 타입으로 바꿔보면 아래와 같은 모습이 될 것이다.
    fn add_sub<T>(a:T, b:T) -> (T,T) {
        return (a+b, a-b);
    }
// 위 코드를 컴파일 해보면 에러가 뜬다. 에러의 내용은,

    error[E0369]: cannot add `T` to `T`
    ...
    help: consider restricting type parameter `T`
        |
    111 | fn add_sub<T: std::ops::Add<Output = T>>(a:T, b:T) -> (T,T) {
    ...
    error[E0369]: cannot subtract `T` from `T`
    ...
    help: consider restricting type parameter `T`
        |
    111 | fn add_sub<T: std::ops::Sub<Output = T>>(a:T, b:T) -> (T,T) {

// 모든 타입의 변수가 +와 - 연산을 지원하는 것이 아니니,
// 이 두 연산을 지원하는 트레잇을 지원하는 타입만 허용하도록 제네릭 타입 T를 제한하라는 거다.
// +연산을 지원하는 트레잇은 std::ops::Add이고 - 연산을 지원하는 트레잇을 std::ops::Sub이다.
// 제네릭에 타입에 대해서, 이 두 개의 트레잇을 지원하는 타입만 가능하도록 '트레잇 바운드'를 하자.

// 트레잇 바운드는 제네릭 타입을 정의하는 곳에 fn add_sub(T: std::ops::Add ....처럼 할 수도 있으나,
// 구문이 길어지는 경우는 where를 쓰는 것이 가독성이 좋다.
// 여러 개의 트레잇을 쓰는 경우는 +로 연결해서 그 다음 트레잇 이름을 쓰면 된다.

    fn add_sub<T>(a:T, b:T) -> (T,T)
        where T: std::ops::Add + std::ops::Sub
    {
        return (a+b, a-b);
    }
// 위 코드에서 트레잇 바운드를 지정했다. 그럼에도 불구하고 컴파일하면 에러가 난다.
// a+b의 결과 타입이 'T'라야 하는데 'associated type'이 결과 타입이라는 얘기다.

    error[E0308]: mismatched types
    ...
    114 |     return (a+b, a-b);
        |             ^^^ expected type parameter `T`, found associated type
// 여기서 std::ops::Add트레잇이 어떻게 정의되어 있는지 알아보자. https://doc.rust-lang.org/ 에서 찾아보면 나온다.

    pub trait Add<Rhs = Self> {
        type Output;

        // Required method
        fn add(self, rhs: Rhs) -> Self::Output;
    }
// 리턴되는 값이 Self::Output으로 연관 타입(associated type)이다. (연관 타입에 대한 설명은 이 페이지 참조. )
// 이처럼 리턴되는 것이 트레잇 내부에 명시된 연관 타입이면,
// 이 트레잇을 사용할 때 해당 연관 타입이 실제로 어떤 타입인지를 결정해줘야 한다.
// 이 예제에서는 add의 결괏값이 T다. 따라서 <Output = T>라고 해주면 된다.
// Sub에 대해서도 마찬가지다.

// 수정된 코드는 다음과 같다.

    fn add_sub<T>(a:T, b:T) -> (T,T)
        where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>
    {
        return (a+b, a-b);
    }
// 이렇게 '연관 타입'까지 지정했는데도 에러가 뜬다. (다행히도 어디를 고쳐면 되는지 컴파일러가 알려주기는 한다. )

    error[E0382]: use of moved value: `a`
    ...
    114 |     return (a+b, a-b);
        |             ---  ^ value used here after move
        |             |
        |             `a` moved due to usage in operator
// a+b를 하면서 a의 소유권이 이동되었다는 거다.

// 위 쪽에 Add 트레잇에 정의된 'add'메서드를 보면 fn add(self, rhs: Rhs) -> Self::Output;로 self를 사용했다.
// 따라서, 이 메서드를 호출한 인스턴스 및 오른 편에 있는 인스턴스의 소유권이 그 결괏값으로 이동될 것이다.
// 따라서, 'a+b'를 하게되면 a와 b의 소유권이 이동된다.

// 만약 a, b의 타입이 i32라고 하면 스택에 저장되는 값이기에 소유권이 이동하거나 하진 않을 것이다.
// 그러나, 지금 우리가 다루는 타입은 제네릭이다. 스택 타입이 올 지 힙에 저장되는 타입이 올 지 모른다.
// 따라서, 컴파일러는 힙 타입도 올 수 있다는 가정하에, 소유권 이동이 발생하는 케이스에 대한 에러를 내는 것이다.

// 컴파일러가 제시하는 해법은 T 타입에 대해 Copy 트레잇을 바운딩하는 것이다.

// Copy 트레잇은, 이것을 구현한 타입들의 연산 결과를 복사해서 넘겨준다.
// 따라서, a+b의 결괏값도 '복사'되기에 a와 b의 소유권이 넘어가지 않는다.
*/

fn add_sub<T>(a: T, b: T) -> (T, T)
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    return (a + b, a - b);
}

fn main() {
    let a: i32 = 2;
    let b: i32 = 3;
    let c: i32 = min(a, b);
    println!("{}", c);

    let a: f32 = 2.5;
    let b: f32 = 3.5;
    let c: f32 = min_generic(a, b);
    println!("{}", c);

    let a: i32 = 2;
    let b: i32 = 3;
    let (c, d) = add_sub(a, b);
    println!("{} {}", c, d);

    let a: f32 = 2.5;
    let b: f32 = 3.5;
    let (c, d) = add_sub(a, b);
    println!("{} {}", c, d);
}
