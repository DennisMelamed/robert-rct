use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    u64
};
extern crate rand;
extern crate crypto;
use rand::{Rng, SeedableRng};
use rand_mersenne_twister::mt64::MTRng64;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn as_u64_unchanged(array: &[u8; 32], result: &mut [u64;32]){
    for i in 0..32
    {
        result[i] = array[i].into(); 
    }
}

fn main() {
    let input_file = std::env::args().nth(1).expect("no input file");
    let output_file = std::env::args().nth(2).expect("no output file");
    println!("Generating Assignments For: {}", input_file);
    let lines = lines_from_file(input_file);
    let num_participants = lines.iter().count();
    let mut full_str: String = "".to_string();
    for line in &lines
    {
        full_str.push_str(&line);
        full_str.push_str("\n");
    }
    full_str.pop(); //get rid of the last newline
    let mut hasher = Sha256::new();
    let mut result: [u8;32] = [0;32];
    hasher.input_str(&full_str);
    hasher.result(&mut result);

    let mut result_64: [u64;32] = [0;32];
    as_u64_unchanged(&result, &mut result_64);
    
    //This is where I think the difference between this code and the one at
    //https://github.com/robert/robert-for-rcts is
    //I think the seed is being interpreted differently
    let mut rng: MTRng64 = SeedableRng::from_seed(&result_64 as &[u64]);

    let mut selections = Vec::new();
    for _p in 0..num_participants
    {
        let random_num = rng.gen_weighted_bool(2);
        if random_num
        {
            selections.push("TREATMENT");
        }
        else
        {
            selections.push("CONTROL");
        }
    }
    let mut output_file_handle = File::create(&output_file).expect("Unable to create file");
    for name_and_selection in lines.iter().zip(selections.iter_mut())
    {
        let (name, selection) = name_and_selection;
        write!(output_file_handle, "{}, {}\n", name, selection).expect("problem writing to file");
    }
    

    println!("Output Assignments To: {}", output_file);
}
