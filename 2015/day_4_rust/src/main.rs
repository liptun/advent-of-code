use std::{fs, io::Error};

fn generate_hash_for(key: &str, value: u64) -> String {
    let input = format!("{}{}", key, value);
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

fn main() -> Result<(), Error> {
    let secret_key_read = fs::read_to_string("input.txt")?;
    let secret_key = secret_key_read.trim_end();

    let mut i: u64 = 0;
    let mut hash: String;
    loop {
        hash = generate_hash_for(&secret_key, i);
        if hash.starts_with("00000") {
            break;
        }
        i += 1;
    }

    println!("Part 1: lowest number is {} that generates that [{}] hash", i, hash );
    
    i = 0;
    loop {
        hash = generate_hash_for(&secret_key, i);
        if hash.starts_with("000000") {
            break;
        }
        i += 1;
    }

    println!("Part 2: lowest number is {} that generates that [{}] hash", i, hash );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hash_for() {
        assert!(
            generate_hash_for("abcdef", 609043).starts_with("000001dbbfa"),
            "test"
        );

        assert!(
            generate_hash_for("pqrstuv", 1048970).starts_with("000006136ef"),
            "test"
        );
    }
}
