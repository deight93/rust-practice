// 구조체의 필드를 제네릭 타입으로 정의할 수 있다. 이렇게 하면 필드의 타입을 사용 시점에 정할 수 있다.

// 앞에서도 예를 들었던 (x,y) 좌표를 필드로 가지는 Point 구조체
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Point 구조체에 제네릭 타입을 적용
struct PointGeneric<T> {
    x: T,
    y: T,
}

// 구조체 메서드에 대한 제네릭 적용
// Point 구조체에서 제네릭 타입을 사용했기에, 이에 대한 impl에서도 제네릭 타입을 사용해야할 것이다.
// 또한, 두 Point를 더한 결과도 Point<T> 타입

impl<T> PointGeneric<T> {
    // fn add(&self, rhs: &PointGeneric<T>) -> PointGeneric<T> {
    //     let x_val = self.x + rhs.x;
    //     let y_val = self.y + rhs.y;
    //     // 위와 같이 add 메서드를 구현해서 컴파일하면 에러가 뜬다.
    //     // self.x + rhs.x라고 한 부분과 self.y + rhs.y라고 한 부분에서의 에러이고,
    //     // Point에서의 x,y가 제네릭 타입이기에 +연산을 지원하는 타입만으로 타입 제한을 해야한다.
    //     // 이것은 일종의 트레잇 바운드이고, + 연산을 지원하는 트레잇을 구현한 타입만을 허용해야하는 것이다.
    //     PointGeneric { x: x_val, y: y_val }
    // }
    // add의 파라미터를 보면, 첫 번째가 &self다. 이 메서드를 호출한 Point 인스턴스가 되겠고,
    // &를 붙였기에 참조 형태로 사용될 것이다. 두 번째 파라미터도 Point<T>형태로 동일
    fn add(&self, rhs: &PointGeneric<T>) -> PointGeneric<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        let x_val = self.x + rhs.x;
        let y_val = self.y + rhs.y;
        PointGeneric { x: x_val, y: y_val }
        // Add 트레잇에는 Output이라는 연관 타입이 있기에, 이 타입 지정을 해야한다는 에러
        // 트레잇 바운드를 할 때, 해당 트레잇의 연관타입을 지정하는 것은 <Output = T>처럼,
        // 해당 연관 타입에 대해 특정 타입으로 지정한다는 것을 명시

        // 위 코드에 대해 여전히 에러가 발생한다.
        // 이번에는 self.x와 rhs.x 등이 '소유권 이동'이 발생해서 다시 사용할 수 없다는 에러다.
        //
        // 이것을 회피하는 제일 간단한 방법은 제네릭 <T>에 대해 Copy 트레잇으로 바운드 하는 거다.
        // 즉, Copy 트레잇을 구현한 타입만으로 T를 제한하는 것이고,
        // Copy 트레잇은 모든 연산 결과를 비트단위로 복사해서 결과값에 전달하기에, 소유권 이동이 일어나지 않는다.
    }
}

// 제네릭 구조체에 대해 타입 지정하여 구현
impl PointGeneric<i32> {
    fn add2(&self, rhs: &PointGeneric<i32>) -> PointGeneric<i32> {
        let x_val = self.x + rhs.x;
        let y_val = self.y + rhs.y;
        PointGeneric { x: x_val, y: y_val }
    }
    // 기본형인 타입인 i32만을 고려해서 구현하면 연산에 대한 고려 및 소유권 이동에 대한 고려가 필요없게 되었다.
    // i32는 덧셈 연산이 지원되고, 기본형 타입이기에 스택 메모리를 사용하기에
    // 소유권 이동이 발생하지 않는 다는 것을 컴파일러가 이미 알고 있어서 소유권 이동에 대한 에러가 발생하지 않는다.
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    println!("{:?}", p1);

    let p2 = PointGeneric { x: 2, y: 3 };
    let p3 = PointGeneric { x: 2.5, y: 3.5 };
    let p4 = PointGeneric { x: 3, y: 4 };
    println!("p2=({},{})", p2.x, p2.y); //p2=(2,3)
    println!("p3=({},{})", p3.x, p3.y); //p3=(2.5,3.5)
    println!("p4=({},{})", p4.x, p4.y); //p3=(3,4)

    // let p5 = p2.add(&p3); //error p1.add(&p2)는 에러가 난다. p1과 p2의 타입이 다르기 때문
    let p5 = p2.add(&p4);
    println!("p4=({},{})", p5.x, p5.y); //p4=(5,7)

    let p6 = p2.add2(&p4);
    println!("p4=({},{})", p6.x, p6.y); //p4=(5,7)
}

// 제네릭 타입을 고려해서 메서드를 구현하면, 구현 복잡성은 높아지나 모든 타입을 지원할 수 있는 장점이 있다.
// 반면 특정 타입만을 고려한 구현을 하면, 구현 복잡성은 낮아지나,
// 해당 타입만을 사용할 수 있다는 단점이 있다. 상황에 따라 선택해서 구현하면 되겠다.
