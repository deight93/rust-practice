// 함수의 리턴 타입에 대해 어떤 트레잇을 구현한 여러 객체가 가능하도록 하려면 Box<dyn Trait>
// 'dynamic'이라는 의미의 dyn 키워드에서 알 수 있듯이,
// 코드가 실행되는 런타임에 리턴되는 타입을 바인딩해서 수행하는 동적 디스패치
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

fn run(x: impl Moving) {
    //  x는 Moving을 impl한 모든 것
    // 이 코드에서 Moving을 impl한 것은 Human과 Dog다. 따라서, x에는 Human과 Dog 객체가 올 수 있다.
    x.run()
}

fn find_runner(is_human: bool) -> Box<dyn Moving> {
    // 위 코드에서 dyn Moving 타입을 감싼 Box를 사용한 이유는,
    // Box가 스마트 포인터로서 해당 객체를 힙에 위치시키고 그 주소를 가리키는 포인터 역할을 하기 때문

    // dyn Moving의 참조자는 인스턴스 객체를 위한 포인터와 vtable(vitual table)을 가리키는
    // 포인터의 총 두 개의 포인터를 갖는다. 이 상태에서 런타임에 이 함수가 수행되서 실제 객체가 리턴될 때
    // 해당 객체에 해당하는 포인터를 vtable에서 찾아서 리턴하게 된다. 동적 디스패치가 일어나는 것이다.
    if is_human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}

fn main() {
    let x = find_runner(true);
    x.run();
    // x의 타입은 Box<dyn Moving>이다. 따라서, x.run()을 하게되면 Box가 담고 있는 객체로
    // 자동 역참조(dereferencing)되면서 그 객체의 메서드인 run()이 호출되게 된다.
    // Box가 담고 있는 것은 Moving 트레잇을 구현한 객체이고, 여기서는 Human 객체가 된다.
    // 즉, 최종 결과는 A human is running.이 출력되게 된다.

    // let x = find_runner(true);
    // run(x);  // 에러
    /*
    여기서 주의할 것은, let x = find_runner(true);로 얻어낸 x를 가지고 run(x)를 할 수는 없다는 것이다.

    run 함수는 fn run(x: impl Moving)으로 선언되어 있기에 파라미터로 Moving을 받을 수는 있으나,
    현재 x의 타입인 Box<dyn Moving>을 받을 수는 없기 때문이다.
    따라서, x.run() 처럼 Moving 트레잇의 메서드를 직접 호출해야 한다.
     */
}
