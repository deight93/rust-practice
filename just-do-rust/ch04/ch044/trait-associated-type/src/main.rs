// 트레잇이 함유하는 항목이 [타입, 상수, 메서드]임
// 타입 부분이 '연관 타입(Associated Type) 이다.
// 트레잇 안에 정의된 타입이고, 트레잇과 '연관'되어 있어서 연관 타입

/*
연관 타입 사용 이유
왜 사용하는 걸까?

Rust 라이브러리 중 연관 타입을 사용한 트레잇을 살펴보면, 왜 사용되었는지를 아는데 도움이 될 것이다.
Rust에 보면 Iterator라는 트레잇이 있다. 정의는 다음과 같이 되어 있다. type Item이라고 되어 있는 것이 연관 타입이다.

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    //...

- next 메서드를 보면 'Self::Item' 타입의 값을 Option 형태로 리턴한다. 여기서 Self는 이 트레잇을 구현한 객체의 타입이다.
 */

fn main() {
    println!("Hello, world!");
}
