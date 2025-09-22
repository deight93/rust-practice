// 어떤 타입이 특정 트레잇을 구현했다면, 그 타입은 해당 트레잇에 선언되어 있는 메서드를 지원한다는 것이 보장된다.
// 이것을 바꿔 얘기하면, 어떤 인스턴스 a가 어떤 메서드 x를 지원하는지는,
// 그 인스턴스 a의 타입이 메서드 x가 선언되어 있는 트레잇을 구현했는가를 보면 된다.

// 이게 어떤 얘기인가 하면, 어떤 구조체 인스턴스 a에 대해 a.clone() 처럼
// a의 인스턴스에서 메서드 clone을 수행할 수 있으려면,
// 해당 구조체가 clone() 메서드를 정의해 놓은 Clone 트레잇을 구현해야한다는 얘기다.

fn main() {
    /*
    표현이 좀 헷갈릴 수도 있는데, 실제 예를 살펴보면 명확하게 이해될 것이다.

    예를 들어보자.

    - x,y라는 변수를 포함하고 있는 구조체인 Point를 만든다.
    - PointOp라는 트레잇을 정의한다. 이 트레잇에는 len라는 메서드를 선언한다.
    - Point에 대해 PointOp 트레잇을 구현(impl)한다.
        여기서 len 메서드의 구현은, 원점 (0,0)에서 (x,y)까지의 거리로 정의한다.
        원점에서 (x,y)까지 거리 = (x.powi(2) + y.powi(2)).sqrt();
    - print_distance(p:&Point)라는 함수를 만든다. 원점에서 p까지의 거리를 출력하는 함수다.
     */

    struct Point {
        x: i32,
        y: i32,
    }
    trait PointOp {
        fn len(&self) -> f64;
    }
    impl PointOp for Point {
        fn len(&self) -> f64 {
            ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
        }
    }

    // fn print_distance(p: &Point) {
    //     println!("Distance={}", p.len());
    // }
    // 위 코드에서 print_distance 함수는 Point 타입만 파라미터로 받게되어 있다.
    // 이것을 다른 타입도 받을 수 있게 하려면 제네릭 타입을 사용하면 된다. 어떤한 타입도 파라미터로 받을 수 있게 하는 것이다.
    // fn print_distance<T>(p: &T) {
    //     println!("Distance={}", p.len()); //error
    // }
    // p.len()이라고 한 부분에 있어, len() 메서드를 찾을 수 없다는 거다.
    // len() 메서드는 PointOp 트레잇에 선언한 것이어서,
    // 이 트레잇을 구현한 Point 타입에 대해서는 p.len()이라는 것이 가능하지만,
    // 제네릭 타입인 T는 모든 타입을 얘기하는 것이고
    // 모든 타입에 len() 메서드가 있는 것은 아니기 때문에 에러가 발생하는 것이다.

    // 제네릭 T 타입에 대해서, len() 메서드가 있는 타입만 수용하는 것으로 한정(bound)
    fn print_distance<T: PointOp>(p: &T) {
        // 제네릭 타입 T인데, PointOp 트레잇을 구현한 것만
        println!("Distance={}", p.len());
    }
    // 이러한 것을 트레잇 바운드라고 한다.
    // 제네릭 타입을 사용할 때, 그 타입을 한정하는 것이다. 어떤 트레잇을 구현한 타입만 받아들이겠다는 것

    let point = Point { x: 2, y: 3 };

    print_distance(&point);

    /*
    트레잇 바운드의 where 표현
    트레잇 바운드는 제네릭 타입을 정의하는 부분에 <T: PointOp>라고 할 수도 있고 where를 써서 표현할 수 도 있다.

    fn min<T>(a:T, b:T) -> T
        where T: std::cmp::PartialOrd {
          ...
    }
     */
}
