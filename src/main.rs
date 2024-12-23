#[allow(unused)]
use my_project::alu::alu;
use my_project::gates::not;

mod test_gates;
mod test_alu;
fn main() {
    println!("Running main ");
    println!("{}",not(1));
    println!("{}", not(0));
}








