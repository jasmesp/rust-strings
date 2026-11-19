use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let mut x = String::new();
    println!("karina!");
    stdin().read_line(&mut x).unwrap();
    println!("x: {}", x);
    if (x.contains(("winter"))) {
println!("o yah thur bud");
}else{
    println!("no, try minjeong!");
} 
}
