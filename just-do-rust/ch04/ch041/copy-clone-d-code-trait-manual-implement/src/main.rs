fn main() {
    // D. Copy 트레잇의 수동 구현

    // Copy 트레잇이 자동 구현되어 있지 않은 타입에 대해서 수동으로 Copy 트레잇을 구현하면,
    // 할당 연산자에 의해서 'move'가 아니라 'copy'가 일어나게 할 수 있다.

    #[derive(Debug, Copy, Clone)]
    // Debug속성은 {:?}에 의해서 출력이 가능하도록하는 것이고,
    // Copy, Clone은 Copy가 일어나도록
    struct Foo;

    let st1 = Foo;
    let st2 = st1;
    println!("00{:?}", st1);
    println!("11{:?}", st2);

    /*
    왜 Copy 트레잇을 구현하려는데 Copy, Clone처럼 Clone 속성도 부여해야하는 걸까?

    Copy 트레잇이 Clone을 슈퍼 트레잇으로 지정해서 정의되어 있기 때문이다.
    즉, Copy 트레잇으로 선언할려면 Clone 트레잇도 구현되어 있어야한다.
    따라서, #[derive(Copy)]는 에러가 난다. #[derive(Copy, Clone)]과 같이 해야한다.

    // Copy 트렛잇의 선언부
    pub trait Copy: Clone { }
     */

    /*
    #[derive(Clone)]은 가능한가?

    만약 #[derive(Clone)]으로 Clone 속성만을 부여하면 어떻게 될까?
    let st2 = st1;이라고 하면 여전히 move가 일어난다.
    그러나, let st2 = st1.clone();과 같이 명시적으로 clone을 한다는 것을
    st1.clone()메서드를 써서 나타내면, st1의 데이터가 모두 복제되서 새로운 메모리에 위치하게 되고
    그 데이터가 st2에 할당되서, 이 후에도 st1의 사용이 가능하다.

    즉, let st2 = st1;이라는 할당 연산자에 의해서 'copy'가 일어나게하려면
    Copy 트레잇이 구현되게끔 해야하고, 따라서 #[derive(Clone)] 만으로는 안된다.
     */

    #[derive(Debug)]
    struct Bar;

    // Copy는 구현할 메서드가 하나도 없는 트레잇이다. 이런 것을 Rust에서는 'marker Trait'이라고 부른다.
    // 구현할 메서드가 없기에 impl할 때는 { }만 추가하면 된다.
    impl Copy for Bar {}

    //  Copy는 Clone을 계승하기 때문에, Copy 트레잇을 impl하기 위해서는 전제 조건으로 Clone을 impl 해야한다.
    impl Clone for Bar {
        // Clone 트레잇에서 구현해야할 메서드는 clone 이다.
        fn clone(&self) -> Bar {
            // *self를 리턴하는 것이기에, 이 트레잇을 구현한 '객체의 값'을 리턴하게 된다.
            // 즉, 객체 데이터를 리턴하는 것이고 이 데이터가 비트 단위로 복제될 것
            *self
        }
    }

    let st3 = Bar;
    let st4 = st3;
    println!("22{:?}", st3);
    println!("33{:?}", st4);
}
