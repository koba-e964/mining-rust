extern crate crypto;
extern crate rand;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use rand::distributions::{IndependentSample, Range};
use std::time::{Instant};


const LEN: usize = 18; // the length of suffixes
const ITERATION: u64 = 1 << 32; // the number of iterations

fn calculate_hash(prefix: &str, suffix: &[u8], buffer: &mut [u8; 20]) {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // write input message
    hasher.input_str(prefix);
    hasher.input(suffix);

    // read hash digest
    hasher.result(buffer);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        eprintln!("Supply an argument!!");
        return;
    }
    let initial_str = &args[1];
    let mut min_hash = [0; 20];
    min_hash[1] = 1;
    let mut rng = rand::thread_rng();

    let numero = Range::new(48, 58);
    for i in 0 .. 16 {
        let num_iter = (2 * i + 1) * ITERATION / 256;
        let start = Instant::now();
        let mut buffer: [u8; 20] = [0; 20];
        for _ in 0 .. num_iter {
            let mut suffix = [0; LEN];
            macro_rules! display {
                () => ({
                    print!("SHA1({}{}) = ",
                           initial_str,
                           String::from_utf8(suffix.to_vec()).unwrap());
                    for i in 0 .. 20 {
                        print!("{:02x}", buffer[i]);
                    }
                    println!("");
                });
            }
            for i in 0 .. LEN {
                suffix[i] = numero.ind_sample(&mut rng) as u8;
            }
            calculate_hash(initial_str, &suffix, &mut buffer);
            if min_hash > buffer {
                print!("Hash update: ");
                display!();
                min_hash = buffer;
            } else if &buffer[0..7] < &[0, 0, 0, 0, 0, 0, 4] {
                print!("Jackpot ");
                display!();
            }
        }
        let end = start.elapsed();
        let end = end.as_secs() as f64 +
            end.subsec_nanos() as f64 * 1e-9;
        println!("Round {}: {}sec", i, end);
        println!(" {} hashes were produced. speed: {} MH/s",
                 num_iter, num_iter as f64 / end / 1.0e6);
    }
}
