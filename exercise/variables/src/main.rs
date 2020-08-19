const MISSILES: i32 = 8;
const READY: i32 = 2;

fn main() {
    let (mut missiles, ready): (i32, i32) = (MISSILES, READY);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left.", missiles - ready);
}
