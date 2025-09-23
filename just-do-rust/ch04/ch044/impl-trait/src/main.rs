// impl Trait 기법은 앞에서 알아본 '트레잇 바운드'와 유사하게 정적 바인딩을 해서 다형성이 가능하게 하는 기법이다.

/*
트레잇 바운드 vs. impl Trait

'트레잇 바운드'는 제네릭 타입에 대해 특정한 트레잇을 구현한 타입만으로 한정하는 기법이다.
반면에 'impl Trait'은 함수의 파라미터나 리턴 타입을 특정 트레잇을 구현한 타입만 가능하도록 하는 기법이다.

둘 다 '특정 트레잇으로 한정'하는 것은 동일하고, '트레잇 바운드'는 제네릭 타입에 대해 한정하는 것이고,
'impl Trait'은 함수의 파라미터나 리턴 타입을 직접 한정하는 것이다.
 */

// impl Trait은 아래 두 위치에서만 사용할 수 있다.
// 함수의 파라미터
// 함수의 리턴
// ------------------------------------------------------------

// 1. 파라미터 위치에 사용되는 경우
//
// 1) Moving이라는 트레잇을 만든다. 이 트레잇에 run이라는 메서드를 정의한다.
// 2) Human과 Dog라는 구조체를 만든다.
// 3) Human과 Dog 구조체에 대해 각각 Moving 트레잇을 impl 한다.
// 4) Human 구조체의 한 객체로 tom을, Dog의 한 객체로 hodu를 생성한다.

struct Dog;
struct Human;

trait Moving {
    fn run(&self);
}

impl Moving for Dog {
    fn run(&self) {
        println!("This dog is moving!");
    }
}

impl Moving for Human {
    fn run(&self) {
        println!("This Human is moving!");
    }
}

// 이제 '움직일 수 있는 모든 것'은 모두 '달리게'하는 run이라는 함수를 만들어보자.
// run(tom) 혹은 run(hodu)와 같이 사용할 수 있는 함수를 만들자는 거다.
// 여기서 '움직일 수 있는 모든 것'은 트레잇 Moving을 의미하는 것이다. 정확하게는 Moving을 impl한 모든 것이다.

// 따라서, 이런 경우는 run 함수의 파라미터로 'Moving을 구현한 것'이라고 한정하면 된다.
// 그렇게 하는 표현이 fn run(x: impl Moving)이다.
fn run(x: impl Moving) {
    //  x는 Moving을 impl한 모든 것
    // 이 코드에서 Moving을 impl한 것은 Human과 Dog다. 따라서, x에는 Human과 Dog 객체가 올 수 있다.
    x.run()
}

// fn run_human(x: Human) {
//     x.run();
// }
//
// fn run_dog(x: Dog) {
//     x.run();
// }

// 트레잇 바운드를 이용한 구현
fn run_trait_bound<T: Moving>(x: T) {
    x.run();
}
// impl Trait 방법: 파라미터의 타입을 해당 Trait를 구현한 객체만으로 하도록 한정하는 것.
// 트레잇 바운드: 제네릭 타입중에서 지정된 Trait를 구현한 객체만으로 한정하는 것

// ------------------------------------------------
// 2. 리턴 위치에 사용되는 경우

// impl Trait은 함수의 리턴 타입에도 사용될 수 있다.
// 어디에 도착한 Human을 리턴하는 함수 who_moved_there를 구현한 것이다.

fn who_moved_there() -> impl Moving {
    // 여기서 fn who_moved_therer()
    // -> Human 이라고 하지 않고
    // -> impl Moving이라고 했다. Moving 트레잇을 구현한 객체를 리턴한다는 의미다.
    // 이 함수를 호출할 때는 아래와 같이 하면된다.
    //     let w = who_moved_there();
    //     run(w);

    let jeff = Human;
    jeff
}
// 이렇게 정확한 객체 타입이 아닌, impl Moving 처럼
// 어떤 트레잇을 구현한 모든 것을 사용할 수 있게 함으로써 프로그램 설계에 유연성을 줄 수 있다.

// 그러나, 사실 많은 유연성을 주진 않는다. 함수의 리턴 타입으로 어떤 트레잇을 구현한 것을 리턴할 수 있다는 것이,
// 이 함수의 내부에서 if 혹은 switch 구문이 있고 그 조건에 의해서,
// 리턴 타입의 트레잇을 구현하기만한 객체면 어떠한 것이라도 리턴할 수 있다는 것을 의미하지 않기 때문이다.

fn find_runner(is_human: bool) -> impl Moving {
    // find_runner라는 함수로, is_human이라는 파라미터에 따라 Human 혹은 Dog 객체를 리턴

    // 위 코드를 보면, Human과 Dog가 Moving 트레잇을 impl하고 있기에,
    // 함수의 리턴 타입을 impl Moving이라고 했기에 문제 없을 것 같다.
    // 그리고, 우리의 목적에도 맞다. Human 혹은 Dog를 경우에 따라서 리턴하고 싶은 것이 이 함수의 목적
    if is_human {
        Human
    } else {
        // Dog // error
        Human
    }

    // 에러가 나는 이유는, 컴파일러가 리턴되는 타입을 결정할 수 없기 때문이다.
    // 위에서 얘기했듯이 Rust는 '정적 바인딩'이 원칙이다.
    // 컴파일 타임에 어떤 타입이 리턴될 지 결정될 수 있어야 한다. 이것은 impl Trait을 사용할 때도 마찬가지다.
    // 위에서 살펴 본 who_moved_there() 함수의 경우 impl Moving을 리턴한다고 되어 있지만,
    // 실제 함수의 내부 코드를 보면 Human 타입만 리턴된다. 따라서, 컴파일러는 Human 타입이 리턴된다고 결정할 수 있다.

    // 그러나, 에러가 난 find_runner 함수의 경우 런타임에서
    // find_runner(true)로 사용될 지 find_runner(false)로 사용될 지 모르기에,
    // 어떤 타입으로 리턴하는 건지 컴파일 타임에는 알 수 없다. 따라서, 컴파일 에러를 내는 것이다.
    // find_runner가 있는 코드를 컴파일 할 때, 에러가 나면서 다음과 같이 에러를 해결할 가이드도 출력된다.

    /*
       95  - fn find_runner(is_human: bool) -> impl Moving {
       95  + fn find_runner(is_human: bool) -> Box<dyn Moving> {
           |
       help: if you change the return type to expect trait objects, box the returned expressions
           |
       102 ~         Box::new(Human)
       103 |     } else {
       104 ~         Box::new(Dog) // error
           |
    */
    // Box<dyn Moving>으로 리턴하라는 거다. 이것은 '동적 바인딩'을 하라는 메시지
}

fn main() {
    let tom = Human;
    let hodu = Dog;

    run(tom);
    run(hodu);
    // run(tom)이라는 코드는 fn run(x: impl Moving)에서 x가 Human 객체이기에,
    // Human이 Moving이 impl한 코드가 수행된다.
    // 이러한 것은 컴파일 타임에 결정된다.
    // 즉, fn run(x: impl Moving)에 대해서 Human과 Dog에 대한
    // 코드가 컴파일러에 의해 자동 생성되고, 런타임에 이 코드가 호출된다.

    let w = who_moved_there();
    run(w);
}

/*
다형성 특성을 구현하기 위해 impl Trait을 함수의 파라미터와 리턴 위치에 사용되는 예를 알아 봤다. 둘 다 정적 바인딩이다.

파라미터 위치에 사용되는 것은 '제네릭 타입 + 트레잇 바인딩'과 유사한 효과를 보이면서,
하나의 코드에 여러 형태의 타입이 사용될 수 있는 전형적인 다형성 특성을 보일 수 있었다.

반면, 리턴 위치에 impl Trait이 사용될 수는 있으나,
이 경우 함수에서 실제로 리턴할 수 있는 타입은 한 가지 타입으로 고정되야해서, 그 쓰임세가 한정적이라는 것을 알았다.
함수 내부에서 여러 타입이 리턴될 수 있으려면 리턴을 impl Trait처럼 하는 것이 아니고 Box<dyn Moving>과 같이 해야한다.
이것은 정적 바인딩이 아니고 런타임에서 바인딩이 되는 동적 바인딩이다. 이에 대해서는 다음 페이지에서 다룬다.
 */
