// Deref는 레퍼런스가 가리키는 곳의 값을 가져오는, 레퍼런스화 되어 있는 것을 원래의 값으로 되돌리는 그러한 트레잇이다.
// 이 트레잇을 구현한 구조체 등이 그렇게 된다는 것이다.

// 예를들어,
// let x = 5;
// let y = &x;
// 라고 되어 있을 때 *y를 하는 것이 역참조(De-reference)이고,
// 이렇게 되게 하는 트레잇이 Deref다.
// 여기서 *y를 하게되면
// *y=*(&x)
// 가 되서, 끝내는
// *y=5
// 가 된다.

// 이처럼 Deref는 De-referencing(역참조)될 때의 행동을 정의하는 것으로만 생각하면 어려울게 없다.
// 그런데 Rust에서는 어떤 경우에 '강제 역참조'가 되는 경우가 있고,
// 이때 이 Deref 트레잇에 있는 deref 함수가 자동 수행되서 이해하기 어렵게 만든다.

// 1. "역참조"의 일반적인 동작
// 어떤 변수 x에 대해 *x에 대한 연산은 다음과 같이 정의 된다.
//
//     *Deref::deref(&x)
//
// 이 연산을 두 개로 분리해서 차례대로 수행해보면,
//
// (1) Deref::deref(&x)
// Dref 트레잇에 정의되는 dref 메서드가 호출된다. Deref 트레잇의 정의는 다음과 같다.
//
//     pub trait Deref {
//         type Target: ?Sized;
//
//         // Required method
//         fn deref(&self) -> &Self::Target;
//     }
//
// 즉, &Self::Target 형태가 리턴된다.
// 만약 x = String::from("abc")와 같디 String 형태라고 해보자.
// String에 대한 Deref에 대한 구현은 아래와 같이 되어 있다.
//
//     impl ops::Deref for String {
//         type Target = str;
//
//         #[inline]
//         fn deref(&self) -> &str {
//             self.as_str()
//         }
//     }
//
// 따라서, Deref::deref(&x)는 &str 타입을 리턴한다.
//
// (2) * 연산
// 이제 위에서 수행한 Dref::deref(&x)에 대해 * 연산을 수행한다.
//
// 위에 예를 들었던 String 타입의 x에 대해 예를 들면, Dref::deref(&x)에 의해 &str 타입이 리턴되었고,
// 이 것에 *연산을 가하면 str형태가 리턴되게 된다.
//
// 즉, x = String::from("abc")에 대해서 *x를 하게 되면 str 타입이 리턴되게 된다.
// 따라서, 실제 코딩을 함에 있어서, y = *x와 같이 String 타입 변수 x를 역참조해서 어떤 변수에 할당할 수가 없다.
// Rust에서 str은 &str에대한 역참조 형태로서 이론적으로 존재할 뿐이지,
// 어떤 변수를 str 타입으로 지정할 순 없다. 이런 이유로 아래 코드는 에러가 뜬다.
//
//     fn main() {
//         let x: String = String::from("abc");
//         let y = *x;  //error
//     }
//
// 2. 스마트 포인터에 대한 Deref
/*
   스마트 포인터는 '실제 데이터는 힙에 있고, 그것을 가리키는 포인터는 스택에 존재하는' 그러한 포인터를 의미하고,
   이러한 것으로 String, Vec, Box 같은 것들이 있다.
   이러한 것들에 대한 객체를 생성한 후, 해당 객체를 가리키는 변수에 대해 *을 붙이면, 이 객체의 데이터가 나와야 한다.
   그것이 우리가 원하는 것이다. 그런데, 스마트 포인터라는 것이 실제 데이터는 힙에 있고,
   변수 정보만 스택에 있기에, 변수에다가 *을 붙였다고 해서 해당 변수가 위치한 스택 메모리의 값을
   리턴해 봐야 원하는 데이터가 나오지 않는다. 힙 데이터가 나와야 한다.
   그래서 스마트 포인터에 대해서는 Deref 트레잇을 구현하게 해서,
   *이 붙여 졌을 때 실제 힙 데이터가 나오도록 별도 조치를 하는 것이다.
*/
//
// 3. 암묵적 "강제 역참조" : 함수/메서드의 파라미터
// Rust에서는 함수나 메서드의 파라미터로 어떤 객체가 전달될 때,
// 메서드가 정의한 파라미터 타입과 맞지 않는 경우 그대로 에러를 내지 않고,
// 넘겨진 객체에 대해 "강제 역참조"를 한 번 시도해서 파라미터 타입과 맞는지 확인해 본다.
// 이러한 부분이 우리를 헷갈리게 한다. 이 부분도 앞 부분에서 설명한 바 있다.
// 그래도 예제 하나를 통해 간단히 알아보면,
// &str 타입을 받는 hello 함수에 대해 &String 타입을 보내도 동작한다는 것이다. 아래 예가 그러한 예다.
//
//     fn hello(name: &str) { // hello 함수는 &str 타입을 파라미터
//         println!("Hello, {}!", name);
//     }
//
//     fn main(){
//         hello("Jeff");
//
//         let s = String::from("Tom");
//         hello(&s);  //success
//     }
//
// &String에 대해서 강제 역참조를 시도하면 &(*String)이 되고,
// String이 구현한 Deref를 보면 str을 리턴하도록되어 있기에 결국 &(*String)=&str이 되어,
// hello 함수가 요구하는 &str 타입에 맞게 되는 것
//
// hello 함수는 &str 타입을 파라미터로 받게 되어 있다.
// 그런데, hello(&s)와 같이 호출을 했고 이 때 s의 타입은 String이기에 &String 형태로 hello를 호출한것이다.
// 그러나, 위에서 설명한 바와 같이, 강제 역참조에 의해 &String이 &str이 되어, 아무 문제없이 수행되는 것이다.
//
// 4. 암묵적 "강제 참조-역참조": 메서드의 호출
// 좀 더 헷갈리긴 하지만 유용하게 쓸 수 있는 상황이 어떤 객체의 메서드를 호출할 때
// 발생하는 '강제 참조-역참조'다. (함수가 아닌 메서드에만 해당한다는 것에 유의)
//
//     Rust 컴파일러는 어떤 객체의 메서드 호출에 대해, 호출된 메서드가 해당 객체에 없는 경우 두 가지를 시도해 본다.
//     1)해당 객체에 대해 참조자를 붙인 후 해당 메서드가 있는지 찾아본다.
//     2)참조자에다가 다시 역참조자를 붙여서 메서드가 있는지 찾아본다.
//
//     쉽게 얘기하면 객체 x에 대한 메서드를 amehod()를 호출할 때 x.amethod()가 없으면,
//     (&x).amehod() 혹은 (*x).amethod를 해본다는 얘기
//
// 1) 해당 객체에 대해 참조자를 붙인 후 해당 메서드가 있는지 찾아본다.
// 예를 들어보자. 위 케이스 1번에 해당하는 예다.
//
//     struct MyStruct {
//         value: i32,
//     }
//
//     impl MyStruct {
//         // This method takes a shared reference to MyStruct
//         fn print_value(&self) {
//             println!("Value: {}", self.value);
//         }
//
//         // This method takes a mutable reference to MyStruct
//         fn increment_value(&mut self) {
//             self.value += 1;
//         }
//     }
//
//     fn main() {
//         let mut my_instance = MyStruct { value: 10 };
//
//         (&my_instance).print_value();
//
//         // Calling print_value: Rust implicitly takes a shared reference (&my_instance)
//         my_instance.print_value();
//
//         // Calling increment_value: Rust implicitly takes a mutable reference (&mut my_instance)
//         my_instance.increment_value();
//         my_instance.print_value();
//     }
//
// `MyStruct`에는 `print_value`와 `increment_value` 메서드가 정의되어 있는데,
// 모두 참조자 &를 사용하도록 정의되어 있다. 즉, `(&my_instance).print_value()`와 같이 호출하도록 되어 있다.
//
// 그러나, `my_instance.print_value()`와 같이 호출해도 문제없이 동작한다.
// MyStruct 구조체에는 `fn print_value(self)`와 같은 메서드가 정의되어 있지 않아서,
// 이와 같은 호출이 문제가 될 것 같지만,
// Rust는 `&my_instance.print_value()`와 같이 `&`를 붙여서 시도해보기에, 문제없이 동작한다.
//
/*
   tip

   Rust에서는 왜 이렇게 만든 것일까?
   어떤 객체에 대한 메서드 호출이라는 것은, 그 객체에 대해 호출하는 것이나, 그것의 참조자 형태로 호출하는 것이나
   똑같은 결과가 나와야하기 때문일 것이다.

   그렇기에, Rust에서 구조체내의 메서드를 호출할 때는, 메서드의 파라미터가 `self`인지 `&self`인지 고민할 필요없다.
   메서드의 파라미터에 `&self`로 되어 있어도 `my_instance.amethod()`처럼 호출이 가능하고,
   메서드의 파라미터로 `self`가 되어 있어도 `my_instance.amehod()` 및
   `&my_instance.amethod()` 둘 다 사용 가능하기 때문이다.
*/
//
// 2)참조자에다가 다시 역참조자를 붙여서 메서드가 있는지 찾아본다.
// 이것은 `역참조자`를 붙였기에 `Deref` 트레잇을 어떻게 구현했는지에 따라 어디서 메서드를 찾을지가 결정되기에,
// 코드를 유연하게 짤 수 있다. (이걸 다른 말로 하면 코드 이해가 어렵게 된다는 의미도 된다. )
//
// 아래 예를 가지고 설명해 보자.
//
// 먼저 Session이라는 구조체를 만들고, 이 구조체용으로 `find`라는 메서드를 만들자.
//
//     struct Session { }
//
//     impl Session {
//         fn find(&self) -> String{
//             "this is find method".to_owned()
//         }
//     }
//
// 이제 이 Session을 변수로 가지는 Driver라는 구조체를 만들자.
// Driver용 메서드는 없고, 이 Driver 구조체에 대해 Deref 트레잇만 구현하도록 하자.
// 다만, Deref 트레잇을 구현할 때 type Target=Session이 되도록 하고
// deref의 리턴값으로 Session이 리턴되도록 하자. 이 부분이 핵심 코드다.
// 역참조를 하게되면 Session 객체가 리턴되도록 하는 것이다.
//
//     struct Driver{
//         handle: Session,
//     }
//
//     impl std::ops::Deref for Driver {
//         type Target = Session;
//         fn deref(&self) -> &Self::Target{
//             &self.handle
//         }
//     }
//
//     //  Driver 객체를 만들고, 이 객체에서 Session에서만 정의되어 있는 메서드를 호출
//     fn main() {
//         let driver = Driver {
//             handle: Session {},
//         };
//         let txt = driver.find();
//         println!("{}",txt);
//     }
// 위 코드에서 Session에만 정의되어 있는 find()메서드를 Driver 객체가 호출했는데도 문제없이 수행된다.
// 이유는 무엇일까? 위 쪽에서 얘기했던 '참조-역참조 강제'가 이루어졌기 때문이다.
//
// 1. driver의 타입인 Driver에 find() 메서드가 있는지 찾아본다. 없다.
// 2. driver에 대해 '참조자'를 붙여본다. &driver가되고 &Driver 타입이다.
//     이 &Driver 타입에 find() 메서드가 있는지 찾아본다. 없다.
// 3. driver에 대해 참조-역참조 강제를 해본다. *&driver가 된다.
//    이것은 Deref::deref(&driver)가 되고 이것의 리턴값은 driver.handle이 된다.
//    즉, driver가 가지고 있는 Session 타입인 handle 객체에 대한 find를 찾게된다.
//    존재한다. 따라서, 이 메서드가 호출된다.
//
// 복잡하게 설명을 하긴 했는데, 쉽게 얘기하면,
//
// 어떤 구조체 A에 대해서, 그 구조체 A가 가지고 있는 어떤 타입 B가 가지고 있는 메서드도 호출할 수 있게 하려면,
// 구조체 A에 대해 Deref를 구현하면서 deref 메서드에서 B의 객체가 리턴되도록 하면 된다.
//
// 어찌보면 굉장히 편리한 기능이다.
// 위 예에서 보면, Driver라는 구조체의 객체에 대해 Session 구조체의 메서드를 사용할 수 있는 것이다.
// 그러나, 코드 분석을 어렵게 만들기도 한다. Driver에는 없는 메서드가 호출되는 것이기에 코드를 분석하는 입장에서는 힘들다.

fn main() {
    println!("Hello, world!");
}
