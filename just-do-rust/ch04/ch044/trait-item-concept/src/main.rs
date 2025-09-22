/*
tip

프로그래밍 발전사를 보면, 프로그램의 규모가 커지면서 한 프로그램에 여러 모듈이 존재하게 되었고,
각 모듈간의 독립성과 공통성이 동시에 요구되었다. 이러한 요구에 가장 부합하는 기법이 객체지향 프로그래밍이었고,
각 객체는 클래스라는 독립 객체로 존재하면서 상속 혹은 인터페이스를 통해 공통의 성질을 공유하게 하였다.
Rust는 구조체(struct)를 Java의 Class와 유사한 객체로 사용할 수 있다.
그러나, Rust에서 구조체끼리의 '상속' 개념은 없다. 만약 구조체 끼리의 공통되는 성질을 정의하려면 트레잇을 사용하면 된다.
즉, Rust의 트레잇은 구조체 혹은 어떤 타입끼리의 공통된 성질(const, type, fn)을 가질 수 있게하기위해 만들어진 기법이다.
 */

// 트레잇에서 항목으로 가질 수 있다는 의미는, 트레잇을 정의할 때 바디에 해당 항목들을 명시할 수 있다는 것이고,
// 이렇게 명시된 항목들은 이 트레잇을 구현하는 타입들에서 공통으로 사용될 수 있다.
// 3가지 항목을 트레잇에 명시한 예이다.

trait ExTrait {
    type TypeNoDefault; //공통으로 사용할 수 있는 타입 정의. 구현측에서 타입지정 해야함

    const CONST_DEFAULT: i32 = 100; //상수값 정의. 구현하는 측에서 재할당 할 수 있음
    const CONST_NO_DEFAULT: i32; //상수 정의만 수행. 구현하는 측에서 값 할당 해야 함

    fn method_default(&self) {
        //함수의 본체까지 정의. 구현 측에서 다시 구현 할 수 있음
        println!("default method");
    }
    fn method_without_default(&self); //공통 메서드 정의. 구현하는 측에서 함수 바디 구현해야함
}
/*
- 공통으로 사용할 수 있는 타입을 정의할 수 있다. 타입에 대해서는 트레잇에서 디폴트 타입을 지정할 순 없다.
- 상수에 대해서는 값을 지정할 수도 있고 안할 수도 있다. 값을 지정하지 않은 상수는 이 트레잇을 구현하는 쪽에서 값을 정의해야 한다.
    이미 지정되어 있어도 구현측에서 값을 재지정할 수 있다.
- 공통으로 사용할 수 있는 메서드를 정의할 수 있다. 함수가 아니고 메서드를 정의하는 것이다.
    따라서, 해당 트레잇을 구현하는 타입들의 인스턴스에서 호출할 수 있는 메서드 들이다.
- 본체가 없이 정의된 메서드는, 이 트레잇을 구현하는 측에서 바디 부분을 구현해야한다. 메서드 본체까지 정의되어 있다면,
    이 메서드는 이 트레잇을 impl하는 타입에서 구현할 의무는 없으나 다시 구현할 수는 있다.
 */

struct ExStruct {}

impl ExTrait for ExStruct {
    // 트레잇 정의에 대해서 구현한 코드
    type TypeNoDefault = f64;

    const CONST_DEFAULT: i32 = 101;
    const CONST_NO_DEFAULT: i32 = 102;

    fn method_default(&self) {
        println!("method is re-implemented");
    }
    fn method_without_default(&self) {
        println!("method_without_default is implemented");
    }
}

// ------------------------------------------

trait Car {
    // Car 트레잇에 drive라는 메서드를 선언만 했다. 바디가 없다.
    // 이런 경우 이 트레잇을 impl하는 모든 것은 이 drive 메서드의 바디를 구현해야한다.
    fn drive(&self);
}

struct Truck {}
impl Car for Truck {
    // drive 메서드의 파라미터로 &self가 사용되었다.
    // 트레잇의 메서드의 파라미터는 대부분 &self를 취한다. 이 트레잇을 구현한 것의 인스턴스를 참조형으로 받겠다는 의미다.
    // 이렇게 참조형 말고 self로 받을 수도 있고 &mut self로 받을 수도 있다.
    fn drive(&self) {
        println!("Truck drive");
    }
}

struct SUV {}
impl Car for SUV {
    fn drive(&self) {
        println!("SUV drive");
    }
}

struct Sedan {}
impl Car for Sedan {
    fn drive(&self) {
        println!("Sedan drive");
    }
}

fn main() {
    // Rust의 트레잇은 타입, 상수, 함수를 항목으로 가질 수 있다.

    // 타입 (type): 공통으로 사용될 수 있는 타입 정의 가능
    // 상수 (const): 트레잇에 정의되어 해당 트레잇을 impl하는 개체들에서 공통으로 사용 가능
    // 함수 (function): 공통으로 사용되는 함수

    // 정의한 트레잇 및 이에 대해 구현한 구조체에 대해 사용할 때는,
    //
    // 타입에 대해서는 let a:<ExStruct as ExTrait>::TypeNoDefault = 1.0;와 같이 사용한다.
    // 상수는 트레잇을 구현한 구조체 이름과 함께 사용된다. ExStruct::CONST_DEFAULT
    // 메서드는 구조체의 인스턴스와 함께 사용된다. s.method_default();

    let s = ExStruct {};
    let a: <ExStruct as ExTrait>::TypeNoDefault = 1.0;
    println!("a={:?}", a);

    println!("CONST_DEFAULT={:?}", ExStruct::CONST_DEFAULT);
    println!("CONST_NO_DEFAULT={:?}", ExStruct::CONST_NO_DEFAULT);

    s.method_default();
    s.method_without_default();

    // 트레잇은 공통으로 사용할 수 있는 상수, 타입, 메서드를 정의한다.
    // 타입을 정의할 수 있다는 부분만 제외하면, Rust의 트레잇은 Java의 인터페이스(interface)와 유사
    // Java의 인터페이스는 상수와 메서드를 정의하고,
    // 이 인터페이스를 implements하는 모든 클래스에서 상수를 공통으로 사용하고 메서드를 구현해서 사용할 수 있다.

    // ------------------------------------

    let car1 = Truck {};
    car1.drive();

    let car2 = SUV {};
    car2.drive();

    let car3 = Sedan {};
    car3.drive();

    // 이처럼, 트레잇은 공통으로 가져야할 메서드를 정의만 해놓는 것이고,
    // 이 트레잇을 impl하는 구조체의 구현부에서 해당 메서드의 바디를 실제 구현한다.
    // 나중에, 이 구조체의 인스턴스에서 해당 메서드를 호출하면 구현된 코드가 수행된다.
}
