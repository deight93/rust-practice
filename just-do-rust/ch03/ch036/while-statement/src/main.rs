fn main() {
    /*
    while <조건식> {
      do_something();
    }
    */
    let mut sum = 0;
    let mut i = 1;

    while i <= 100 {
        sum += i;
        i += 1;
    }
    println!("{}", sum);
}
