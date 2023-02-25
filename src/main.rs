use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let file_name = args.next().ok_or("no second arg")?;

    let f = File::open(file_name.clone())?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    println!("Finding primes");

    let mut is_prime = vec![true; buffer.len()];
    is_prime[1] = false;
    is_prime[0] = false;

    let root = ((buffer.len() as f64).sqrt().round()) as usize + 1;
    for i in 2..root {
        if is_prime[i] {
            let mut multiplier = 0;

            loop {
                let index = i * i + multiplier * i;
                if index >= buffer.len() {
                    break;
                }
                is_prime[index] = false;
                multiplier += 1;
            }
        }
    }
    println!("Flipping bytes");

    let flipped: Vec<_> = buffer
        .into_iter()
        .enumerate()
        .map(|(i, byte)| if is_prime[i] { byte ^ 0xFF } else { byte })
        .collect();

    println!("Writing result");
    let mut result_file = File::create(file_name + ".flipped")?;
    result_file.write_all(&*flipped)?;

    Ok(())
}
