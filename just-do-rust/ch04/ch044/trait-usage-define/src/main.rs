fn main() {
    // 2. 트레잇을 정의해야 하는 경우

    // 이 Point 끼리의 덧셈을 std::ops::Add 트레잇을 구현해서 가능하게 했다.
    // 여기서 약간의 문제는 덧셈을 하고난 후 두 인스턴스의 소유권이 이동한다는 것
    //  그래서, 덧셈 후 뺄셈을 하려면 인스턴스를 복제해서 연산을 해야했다.
    // 이는 std::ops::Add 트레잇에 정의된 add와 sub 메서드에서
    // 참조 타입을 쓰지 않기 때문에 소유권 이동이 일어나서 그렇다.

    // 여기서는, 참조 타입을 쓰는 add sbu 메서드를 만들어보자.
    // 새롭게 MyOp라는 트레잇을 만들고, 거기서 새로운 add sub 메서드를 정의하는 것
    struct Point {
        x: i32,
        y: i32,
    }

    trait MyOp {
        //  std::Ops::Add 트레잇은 trait Add<Rhs = Self>처럼 모든 타입을 지원하기 위해 제네릭이 사용되어 있다.
        // 여기서는 Point에 대해서만 지원하도록 단순하게 선언한 것
        type Output;
        fn add(&self, rhs: &Self) -> Self::Output;
        fn sub(&self, rhs: &Self) -> Self::Output;
        // self와 Self
        // self는 현재의 '인스턴스'를 말하고, Self는 현 인스턴스의 '타입'을 의미한다.
    }

    impl MyOp for Point {
        type Output = Self;

        fn add(&self, rhs: &Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }

        fn sub(&self, rhs: &Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
        // 파라미터는 모두 참조자 형태여서 소유권 이동이 발생하지 않는다.
        // 따라서, 이제 Point 타입 인스턴스에 대해 add sub를 수행할 때 소유권 이동이 발생하지 않을 것이다.
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3 = p1.add(&p2);
    let p4 = p1.sub(&p2);

    println!("{} {}", p3.x, p3.y);
    println!("{} {}", p4.x, p4.y);
}
