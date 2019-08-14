use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
extern crate mersenne_twister;
extern crate rand;
use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let input_file = std::env::args().nth(1).expect("no input file");
    let output_file = std::env::args().nth(2).expect("no output file");
    println!("Generating Assignments For: {}", input_file);
    let lines = lines_from_file(input_file);
    let num_participants = lines.iter().count();
    let mut full_str: String = "".to_string();
    for line in lines
    {
        full_str.push_str(&line);
        full_str.push_str("\n");
    }
    let hash = calculate_hash(&full_str);
    println!("Hash of {} names: {}", num_participants, hash);
    let mut rng: MersenneTwister = SeedableRng::from_seed(hash);

    let mut selections = Vec::new();//vec![String, num_participants, num_participants];
    for _p in 0..num_participants
    {

        let random_num = rng.next_u64();
        if random_num %2 == 0
        {
            selections.push("TREATMENT");
        }
        else
        {
            selections.push("CONTROL");
        }
    }
    let mut output_file_handle = File::create(&output_file).expect("Unable to create file");
    for line in &selections
    {
        write!(output_file_handle, "{}\n", line).expect("problem writing to file");
    }
    

    println!("Output Assignments To: {}", output_file);
}
