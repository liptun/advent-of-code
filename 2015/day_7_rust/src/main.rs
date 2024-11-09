use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};
mod wire_kit;

use wire_kit::{Wire, WiresKit};

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut kit = WiresKit::new();

    for line in reader.lines().filter_map(Result::ok) {
        kit.process_instruction(&line);
    }

    println!(
        "Starts evaluating circuit with {} wires and {} gates",
        kit.wire.iter().count(),
        kit.connections.len()
    );
    match kit.evaluate_circuit() {
        Ok(_) => println!("Evaluation succes"),
        Err(e) => println!("Evaluation error: {:?}", e),
    }

    let wire_a_signal = if let Some(a) = kit.wire.get("a").cloned() {
        a
    } else {
        Wire::Unknown
    };

    match wire_a_signal {
        Wire::Signal(s) => println!("Wire [a] has signal: {}", s),
        Wire::Unknown => println!("Wire [a] is not found"),
    }

    kit.reset_all_wires_expect_inputs();

    kit.wire.insert("b".to_string(), wire_a_signal);
    match kit.evaluate_circuit() {
        Ok(_) => println!("Evaluation succes"),
        Err(e) => println!("Evaluation error: {:?}", e),
    }

    let wire_a_signal = if let Some(a) = kit.wire.get("a").cloned() {
        a
    } else {
        Wire::Unknown
    };

    match wire_a_signal {
        Wire::Signal(s) => println!("Wire [a] has signal: {}", s),
        Wire::Unknown => println!("Wire [a] is not found"),
    }

    Ok(())
}
