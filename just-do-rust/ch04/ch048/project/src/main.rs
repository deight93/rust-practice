// 프로젝트는 'cargo new'에 의해 만들어지는, Rust 프로그램을 만들기 위한 모든 리소스의 모음
// Rust로 어떤 프로그램을 만들고자 할 때, 제일 먼저 하는 작업은 cargo new [프로젝트 이름]이다.

/*
tip

물론 cargo를 이용하지 않고도 Rust 프로그램을 만들 수 있지만, 여기서는 cargo를 이용하는 것을 가정하고 설명한다.
 */

/*
프로젝트의 이름을 backyard라고 한다면 cargo new backyard에 의해서 backyard 폴더가 생성되고
그 폴더 밑에 프로젝트에 필요한 파일들과 src폴더 밑에 main.rs 파일이 자동 생성된다.
또한 프로젝트 폴더에는 Cargo.toml 파일이 있는데, 이 파일이 패키지를 정의해 준다.

 */

fn main() {
    println!("Hello, world!");
}
