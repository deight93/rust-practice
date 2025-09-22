// Rust 프로그래밍을 하면서 트레잇을 사용하게 되는 경우는 크게 두가지.
// 1. Rust 기본 라이브러리에서 요구되는 트레잇을 구현해야 하는 경우
// 2. 개인이 트레잇을 정의해야 하는 경우

fn main() {
    // 1. 트레잇을 구현해야 하는 경우
    // 예를 들어 (x,y) 값을 가지는 Point라는 구조체를 정의했고,
    // Point 끼리의 덧셈을 구현하려고 한다고 해보자. Point 끼리의 덧셈이라는 것은 x끼리 더하고 y 끼리 더하는 것

    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };
    // let p3 = p1 + p2; // error
    // println!("{} {}", p3.x, p3.y);
    // 위와 같은 코드에 대해 Rust 컴파일러는 다음과 같은 에러를 낼 것이다.
    // Point 끼리의 덧셈을 하려면 트레잇 Add를 impl 해야 한다는 것이다.

    // Add 트레잇 구현
    impl std::ops::Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Clone for Point {
        fn clone(&self) -> Self {
            Self {
                x: self.x,
                y: self.y,
            }
        }
    }

    let p3 = p1.clone() + p2.clone();
    println!("addition=({},{})", p3.x, p3.y); //plus=(6,8)

    // Sub 트레잇 구현
    // let p4 = p1 - p2; // error -> note: the trait `Sub` must be implemented
    // println!("subtract=({},{})", p4.x, p4.y);

    // 뺄셈은 std::ops::Sub 트레잇을 impl 하면 된다.
    impl std::ops::Sub for Point {
        type Output = Point;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    // let p4 = p1 - p2; // error
    // Add 트레잇을 구현할 때 add 메서드를 보면, 파라미터가 self 및 Self였다. 참조형이 아니다.
    // add를 수행하면, 수행하는 인스턴스들의 소유권이 결괏값으로 이동한다.
    // 따라서, 그 이후에 해당 인스턴스를 사용하려고 하면 이런 에러가 나는 것
    let p4 = p1 - p2;
    println!("subtract=({},{})", p4.x, p4.y);
}
