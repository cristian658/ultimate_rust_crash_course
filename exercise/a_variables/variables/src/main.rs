const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 2;
fn main() {
    //let varNotUse : i32;//What does cargo say when you run your program? warning: unused variable: `varNotUse`
    //READY_AMOUNT = 1;// What does the error look like? error: a constant value cannot be changed!
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    //let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles ...", ready, missiles);
    //missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
