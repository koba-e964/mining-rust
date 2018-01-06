extern crate crypto;
extern crate rand;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use rand::distributions::{IndependentSample, Range};

const LEN: usize = 18; // the length of suffixes
const ITERATION: u64 = 1 << 32; // the number of iterations

fn calculate_hash(prefix: &str, suffix: &[u8]) -> String {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // write input message
    hasher.input_str(prefix);
    hasher.input(suffix);

    // read hash digest
    hasher.result_str()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        eprintln!("Supply an argument!!");
        return;
    }
    let initial_str = &args[1];
    let mut min_hash = "}".to_string();
    let mut rng = rand::thread_rng();

    let numero = Range::new(48, 58);
    for _ in 0 .. ITERATION {
        let mut suffix = [0; LEN];
        for i in 0 .. LEN {
            suffix[i] = numero.ind_sample(&mut rng) as u8;
        }
        let hash = calculate_hash(initial_str, &suffix);
        if min_hash > hash {
            println!("Hash update: SHA1({}{}) = {}",
                      initial_str,
                      String::from_utf8(suffix.to_vec()).unwrap(),
                      hash);
            min_hash = hash;
        } else if &hash[0..6] == "000000" {
            println!("Jackpot SHA1({}{}) = {}",
                      initial_str,
                      String::from_utf8(suffix.to_vec()).unwrap(),
                      hash);
        }
    }
}
