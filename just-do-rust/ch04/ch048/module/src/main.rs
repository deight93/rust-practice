// Rust에서의 모듈은 크레이트를 구성하는 요소로, mod [모듈명]으로 모듈을 선언해서 생성하기도 하고,
// 별도의 파일로 소스코드를 나눠서 별도의 모듈이 되게할 수도 있다

// 프로그램이 커지면 여러 개의 소스 파일로 나눠서 프로그램을 개발하게 된다.
// Rust에서는 각각의 파일이 하나의 모듈이다. 그리고, 한 파일안에서도 mod [모듈명]으로 모듈을 만들 수도 있다.

// 모듈의 선언
// main.rs 파일에서 mod garden { }이라고 하면 main 모듈 밑에 garden 모듈이 생성되는 것이다.
// 이 모듈 안에 코드들을 넣을 수 있고, 또 다른 서브 모듈을 생성할 수도 있다.

// mod garden {
//     // main.rs 파일에서 mod garden { }을 하면 메인 모듈 밑에 garden이라는 모듈이 생성된다.
//     // pub 키워드를 붙이지 않았다. main에서 사용할 것이고,
//     // main 바로 밑에 있는 모듈이기에 pub를 붙이지 않아도 접근이 가능하다.
//     pub mod vegetables {
//         //  'garden' 모듈 밑에 'vegetables' 이라는 서브 모듈을 만든 것이다.
//         // 이 모듈을 메인 모듈에서 접근하려면 pub를 붙여서 선언해야한다.
//         // 왜냐면, garden 바로 밑에 있는 것이기게 garden 모듈에서는 pub가 없어도 접근 가능하지만,
//         // 메인에서는 garden에 막혀서 접근이 안된다. 따라서 pub를 붙여줘야 main에서 접근이 가능하다.
//         #[derive(Debug)]
//         pub struct Asparagus {}
//         // vegetable 모듈 안에서 구조체 하나를 선언했다. 이 구조체도 main에서 접근하려면 pub를 붙여줘야 한다.
//     }
// }

use crate::garden::vegetables;
// main 함수내에서 garden::vegetables::Asparagus {}라고 해도 되지만,
// 이처럼 use를 써서 미리 사용한다고 할 수 있다. 이때 crate를 맨 앞에 쓴 것에 유의한다.
mod garden;
// garden 모듈을 사용하겠다는 의미다. 이렇게 하면 컴파일러는 'src/garden.rs'가 있는지,
// 혹은 src/garden/mod.rs가 있는지를 찾는다.
// 여기서는 'src/garden.rs'가 있기에, 이 파일 안의 내용을 main에서 사용할 수 있게 된다.

/*
위 코드에서 mod garden;이라고 한 것에 대해서 좀 더 설명하면,
이 문장에 대해 Rust 컴파일러는 3군데서 이 garden 모듈을 찾는다.

1) mod garden 뒤에 { }이 있다면, 이 블럭안 코드가 garden 모듈이 된다.
2) 현 위치인 main.rs가 있는 폴더에 garden.rs가 있는지 찾는다.
   여기서는 실제로 이 파일이 있어서, garden.rs 파일 안의 내용이 garden 모듈이 된다.
3) 현 위치의 폴더에서 garden이라는 폴더가 있고
   그 안에 'mod.rs' 파일이 있는지 찾아서, 그 파일 안의 내용이 garden 모듈이 된다.
 */

/*
tip

필자는 위 방법 중 2)번 방법을 선호하고, 간단한 모듈은 1)번 방법을 사용한다.
3)번 방법은 사용하지 않는다. 1번과 2번 방법만으로도 충분하다.
 */

/*
pub

위 코드에서 살펴봤지만 같은 파일내에서 선언된 모듈에 대해서는 pub mod가 아닌 mod로 선언되어 있어도 접근이 가능하지만,
파일이 다른 경우는 해당 모듈이 pub mod로 선언되어 있지 않으면 다른 파일에서 접근이 안된다.
 */

fn main() {
    // let plant = garden::vegetables::Asparagus {};
    // // 메인에서 접근할 때는 garden::vegetables::Asparagus {} 처럼 계층구조를 명시해서 접근한다.
    // // 혹은 use를 써서 계층구조를 미리 사용한다고 선언 후에 사용할 수도 있다.
    // println!("I'm growing {:?}", plant);

    let plant = vegetables::Asparagus {};
    // 'vegetables'는 garden 모듈안에 있는 서브 모듈이다.
    // main에서 이 vegetables 모듈을 접근가능한 것은, mod garden;을 통해서 garden 모듈에 접근이 가능했고,
    // 이 garden 모듈안에 pub mod vegetables;이라고 해서 vegetables를 사용할 수 있게 선언했기에,
    // 결국은 main에서 main > garden > vegetables의 경로를 통해
    // 끝내는 vegetables::Asparagus 구조체를 사용할 수 있게 된 것이다.
    println!("I'm growing {:?}", plant);
}
