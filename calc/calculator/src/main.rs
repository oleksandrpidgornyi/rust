use calclib;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        calclib::add_one(num)
    );
}
