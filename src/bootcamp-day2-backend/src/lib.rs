#[ic_cdk::query]
fn greet(name: String, num: i8) -> String {
    format!("Hello, {}, this is your num:\n{}", name, num)
}
