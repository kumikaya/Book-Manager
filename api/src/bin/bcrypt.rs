
fn main() {
    let hash = bcrypt::hash("admin", bcrypt::DEFAULT_COST).unwrap();
    println!("{}", hash);
    let hash = bcrypt::hash("123456", bcrypt::DEFAULT_COST).unwrap();
    println!("{}", hash);
}