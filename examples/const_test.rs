mod c1 {
    const A: i64 = 100;
    pub fn test() {
        eprintln!("A = {:#?}", A);
    }
}
mod c2 {
    const A: i64 = 200;
    pub fn test() {
        eprintln!("A = {:#?}", A);
    }
}

fn main() {
    c1::test();
    c2::test();
}
