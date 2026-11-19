#![allow(unused_parens)]

use std::io::stdin;

fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    let mut x = String::new();
    println!("karina!");
    stdin().read_line(&mut x)?;
    println!("x: {}", x);
    if (x.contains(("winter"))) {
println!("o yah thur bud");
}else{
    println!("no, try minjeong!");
} 
return Ok(());

}
