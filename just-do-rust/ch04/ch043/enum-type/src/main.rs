// Rust에서 enum의 선언은 구조체와 유사하다. enum 키워드 다음에 이름을 적고 블록 안에 타입들을 열거한다.
enum Gender {
    Male,
    Female,
}

// 'Gender'라는 열거형 정의를 좀 수정해서, 'Male' 타입의 경우는 이름과 군대복무여부 정보를 가지게하고,
// 'Female'의 경우는 이름 정보만 가지게 하자.
// 이처럼 열거형의 원소 타입에 어떤 정보를 더 부가할 수 있다.
enum GenderAddInfo {
    Male { name: String, is_military: bool },
    Female { name: String },
    // 'Male' 타입에 구조체 형태로 name과 is_military 정보를 추가했다.
    // 'Female' 에는 name 정보만 가지는 구조체로 했다.
    // (필드가 하나기에 굳이 구조체 형태가 아니어도 되나, Male과의 구조 유사성을 위해 그냥 구조체로 선언했다. )
}

fn main() {
    // 열거형(enum)은 어떤 대등 관계에 있는 타입들을 원소로 열거하고자 할 때 사용
    let gender = get_customer(10);
    match gender {
        Gender::Male => println!("Male"),
        Gender::Female => println!("Female"),
    }

    /*
    Rust에서는 ::는 스태틱한 필드나 함수에 접근할 때, .은 인스턴스의 메서드나 필드에 접근할 때 사용한다.

    # :: 사용의 경우
    * 구조체의 연관 함수 접근: String::new() 등
    * 열거형의 타입 접근: Option::None , Gender::Male 등

    # . 사용의 경우
    * 구조체 인스턴스의 필드 및 메서드 접근: student.name student.get_name() 등
    * 열거형 인스턴스의 메서드 접근: optoin.ok() 등
    */

    // 열거형을 쓰는 것은 주로 타입 체크 때문
    // 위에서 예를 든 'Male' 'Female'도 그렇지만,
    // 어떤 함수의 리턴값이 있을 때 '값'이 아니라 그 '타입'으로 구분해서 처리를 하고 싶을 때

    let gender2 = get_add_info_customer(10);
    match gender2 {
        GenderAddInfo::Male {
            name: n,
            is_military: b,
        } => {
            println!("name={}, Military={}", n, b);
        }
        GenderAddInfo::Female { name: n } => {
            println!("Female: {}", n);
        }
    }

    // 위에서 살펴본 바와 같이, 열거형은 해당 타입 차원에서 뭔가 구분을 하고 싶을 때 사용한다.
    // Rust에서 디폴트로 제공되고, 매우 많이 사용되는 Option과 Result가 열거형이다.
    // 이 두 가지를 모르면 Rust로 된 코드를 이해하기 힘들다. 다음 페이지에서는 이 두 가지 열거형의 사용법을 소개한다.
}

fn get_customer(id: i32) -> Gender {
    if id % 2 == 0 {
        return Gender::Male; // 열거형의 타입에 접근할 때는 <열거형 이름>::<원소 타입 이름>처럼 접근
    }
    return Gender::Female;
}

fn get_add_info_customer(id: i32) -> GenderAddInfo {
    if id % 2 == 0 {
        return GenderAddInfo::Male {
            name: "Jaff".to_owned(),
            is_military: true,
        };
    }
    return GenderAddInfo::Female {
        name: "Alice".to_owned(),
    };
}
