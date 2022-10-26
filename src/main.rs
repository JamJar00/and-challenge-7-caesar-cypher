use std::time::Instant;
use std::env;

fn main() {
    let mut pnp= std::fs::read("pride_and_prejudice.txt").unwrap();
    let degree = u8::from_str_radix(env::args().nth(1).unwrap().trim(), 10).unwrap();

    let start = Instant::now();
    for i in 1..pnp.len() {
        let c = pnp[i];
        let v;
        if c.is_ascii_uppercase() {
            v = (c - 65 + degree) % 26 + 65;
        } else if c.is_ascii_lowercase() {
            v = (c - 97 + degree) % 26 + 97;
        } else {
            v = c;
        }
        pnp[i] = v;
    }

    let elapsed = start.elapsed();
    std::fs::write("pride_and_prejudice.txt.cyphered", pnp).unwrap();
    println!("Elapsed: {:.2?}", elapsed);
}
