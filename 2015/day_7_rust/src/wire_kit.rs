use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Wire {
    Unknown,
    Signal(u16),
}

#[derive(Debug)]
pub enum Input {
    Wire(String),
    Signal(u16),
}

#[derive(Debug)]
pub enum Output {
    Wire(String),
}

#[derive(Debug)]
pub enum Gate {
    Relay(Input, Output),
    NOT(Input, Output),
    AND(Input, Input, Output),
    OR(Input, Input, Output),
    LSHIFT(Input, u16, Output),
    RSHIFT(Input, u16, Output),
}

trait GateUtil {
    fn get_output(&self) -> &Output;
    fn get_output_wire_name(&self) -> String;
}

impl GateUtil for Gate {
    fn get_output(&self) -> &Output {
        match self {
            Gate::NOT(_, output)
            | Gate::Relay(_, output)
            | Gate::AND(_, _, output)
            | Gate::OR(_, _, output)
            | Gate::LSHIFT(_, _, output)
            | Gate::RSHIFT(_, _, output) => output,
        }
    }
    fn get_output_wire_name(&self) -> String {
        let Output::Wire(wire_name) = self.get_output();
        wire_name.to_string()
    }
}

#[derive(Debug)]
pub enum Process {
    Input,
    Relay,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    Error,
}

#[derive(Debug)]
pub enum EvaluateErrors {
    NotInputError,
    AndInputBError,
    OrInputAError,
    OrInputBError,
    ShiftInputError,
    ShiftDirectionError,
}

pub struct WiresKit {
    pub wire: HashMap<String, Wire>,
    pub connections: Vec<Gate>,
}

impl WiresKit {
    pub fn new() -> Self {
        Self {
            wire: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn reset_all_wires_expect_inputs(&mut self) {
        for gate in self.connections.iter() {
            let wire_name = gate.get_output_wire_name();
            self.wire.insert(wire_name, Wire::Unknown);
        }
    }

    pub fn evaluate_circuit(&mut self) -> Result<(), EvaluateErrors> {
        self.reset_all_wires_expect_inputs();

        loop {
            for gate in self.connections.iter() {
                match gate {
                    Gate::Relay(input, output) | Gate::NOT(input, output) => {
                        let input_wire: String = match input {
                            Input::Wire(input_wire_name) => input_wire_name.to_string(),
                            _ => return Err(EvaluateErrors::NotInputError),
                        };

                        if let Some(input_value) = self.wire.get(&input_wire) {
                            if let Wire::Signal(signal) = input_value {
                                let Output::Wire(output_wire) = output;
                                let result = if let Gate::NOT(_, _) = gate {
                                    !signal
                                } else {
                                    *signal
                                };
                                self.wire
                                    .insert(output_wire.to_string(), Wire::Signal(result));
                            }
                        }
                    }
                    Gate::AND(input_a, input_b, output) => {
                        let input_wire_b: String = match input_b {
                            Input::Wire(wire) => wire.to_string(),
                            _ => return Err(EvaluateErrors::AndInputBError),
                        };

                        let input_value_a = match input_a {
                            Input::Signal(signal) => &Wire::Signal(*signal),
                            Input::Wire(wire) => self.wire.get(wire).unwrap(),
                        };

                        let input_value_b = self.wire.get(&input_wire_b).unwrap();

                        match (input_value_a, input_value_b) {
                            (Wire::Signal(signal_a), Wire::Signal(signal_b)) => {
                                let result = signal_a & signal_b;
                                let Output::Wire(output_wire) = output;
                                self.wire
                                    .insert(output_wire.to_string(), Wire::Signal(result));
                            }
                            _ => {}
                        }
                    }
                    Gate::OR(input_a, input_b, output) => {
                        let input_wire_a: String = match input_a {
                            Input::Wire(wire) => wire.to_string(),
                            _ => return Err(EvaluateErrors::OrInputAError),
                        };
                        let input_wire_b: String = match input_b {
                            Input::Wire(wire) => wire.to_string(),
                            _ => return Err(EvaluateErrors::OrInputBError),
                        };

                        let input_value_a = self.wire.get(&input_wire_a).unwrap();
                        let input_value_b = self.wire.get(&input_wire_b).unwrap();

                        match (input_value_a, input_value_b) {
                            (Wire::Signal(signal_a), Wire::Signal(signal_b)) => {
                                let result = signal_a | signal_b;
                                let Output::Wire(output_wire) = output;
                                self.wire
                                    .insert(output_wire.to_string(), Wire::Signal(result));
                            }
                            _ => {}
                        }
                    }
                    Gate::LSHIFT(input, factor, output) | Gate::RSHIFT(input, factor, output) => {
                        let input_wire: String = match input {
                            Input::Wire(wire) => wire.to_string(),
                            _ => return Err(EvaluateErrors::ShiftInputError),
                        };
                        let input_value = self.wire.get(&input_wire).unwrap();

                        if let Wire::Signal(input_signal) = input_value {
                            let Output::Wire(output_wire) = output;

                            let result = match gate {
                                Gate::LSHIFT(_, _, _) => input_signal << factor,
                                Gate::RSHIFT(_, _, _) => input_signal >> factor,
                                _ => {
                                    return Err(EvaluateErrors::ShiftDirectionError);
                                }
                            };
                            self.wire
                                .insert(output_wire.to_string(), Wire::Signal(result));
                        }
                    }
                }
            }

            let unknown_wires = self
                .wire
                .values()
                .filter(|w| matches!(w, Wire::Unknown))
                .count();
            if unknown_wires == 0 {
                break;
            }
        }

        Ok(())
    }

    pub fn process_instruction(&mut self, s: &str) -> Process {
        let input_pattern = Regex::new(r"^(?P<signal>\d+) -> (?P<wire>[a-z]+)$").unwrap();
        let relay_pattern = Regex::new(r"^(?P<input>[a-z]+) -> (?P<output>[a-z]+)$").unwrap();
        let not_pattern = Regex::new(r"^NOT (?P<input>[a-z]+) -> (?P<output>[a-z]+)$").unwrap();
        let and_pattern = Regex::new(
            r"^(?P<input_a>([a-z]+|\d+)) AND (?P<input_b>[a-z]+) -> (?P<output>[a-z]+)$",
        )
        .unwrap();
        let or_pattern =
            Regex::new(r"^(?P<input_a>[a-z]+) OR (?P<input_b>[a-z]+) -> (?P<output>[a-z]+)$")
                .unwrap();
        let shift_pattern = Regex::new(
            r"^(?P<input>[a-z]+) (?P<direction>(L|R)SHIFT) (?P<factor>\d+) -> (?P<output>[a-z]+)$",
        )
        .unwrap();

        if let Some(caps) = input_pattern.captures(s) {
            let signal: Wire = match caps.name("signal") {
                Some(signal) => match signal.as_str().parse::<u16>() {
                    Ok(parsed_signal) => Wire::Signal(parsed_signal),
                    Err(_) => return Process::Error,
                },
                None => return Process::Error,
            };
            let wire_name: String = match caps.name("wire") {
                Some(wire_name) => wire_name.as_str().to_string(),
                None => return Process::Error,
            };

            self.wire.insert(wire_name, signal);

            return Process::Input;
        };

        if let Some(caps) = relay_pattern.captures(s) {
            let input: Input = match caps.name("input") {
                Some(input) => Input::Wire(input.as_str().to_string()),
                None => return Process::Error,
            };

            let output: Output = match caps.name("output") {
                Some(output) => Output::Wire(output.as_str().to_string()),
                None => return Process::Error,
            };

            let gate = Gate::Relay(input, output);

            self.connections.push(gate);

            return Process::Relay;
        }

        if let Some(caps) = not_pattern.captures(s) {
            let input: Input = match caps.name("input") {
                Some(input) => Input::Wire(input.as_str().to_string()),
                None => return Process::Error,
            };
            let output: Output = match caps.name("output") {
                Some(output) => Output::Wire(output.as_str().to_string()),
                None => return Process::Error,
            };

            let gate = Gate::NOT(input, output);

            self.connections.push(gate);

            return Process::NOT;
        };

        if let Some(caps) = and_pattern.captures(s) {
            let input_a: Input = match caps.name("input_a") {
                Some(input) => match input.as_str().parse::<u16>() {
                    Ok(v) => Input::Signal(v),
                    Err(_) => Input::Wire(input.as_str().to_string()),
                },
                None => return Process::Error,
            };
            let input_b: Input = match caps.name("input_b") {
                Some(input) => Input::Wire(input.as_str().to_string()),
                None => return Process::Error,
            };
            let output: Output = match caps.name("output") {
                Some(output) => Output::Wire(output.as_str().to_string()),
                None => return Process::Error,
            };

            let gate = Gate::AND(input_a, input_b, output);

            self.connections.push(gate);

            return Process::AND;
        };

        if let Some(caps) = or_pattern.captures(s) {
            let input_a: Input = match caps.name("input_a") {
                Some(input) => Input::Wire(input.as_str().to_string()),
                None => return Process::Error,
            };
            let input_b: Input = match caps.name("input_b") {
                Some(input) => Input::Wire(input.as_str().to_string()),
                None => return Process::Error,
            };
            let output: Output = match caps.name("output") {
                Some(output) => Output::Wire(output.as_str().to_string()),
                None => return Process::Error,
            };

            let gate = Gate::OR(input_a, input_b, output);

            self.connections.push(gate);

            return Process::OR;
        };

        if let Some(caps) = shift_pattern.captures(s) {
            let input: Input = match caps.name("input") {
                Some(input) => Input::Wire(input.as_str().to_string()),
                None => return Process::Error,
            };
            let direction: &str = match caps.name("direction") {
                Some(input) => input.as_str(),
                None => return Process::Error,
            };
            let factor: u16 = match caps.name("factor") {
                Some(output) => match output.as_str().parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => return Process::Error,
                },
                None => return Process::Error,
            };
            let output: Output = match caps.name("output") {
                Some(output) => Output::Wire(output.as_str().to_string()),
                None => return Process::Error,
            };

            match direction {
                "LSHIFT" => {
                    let gate = Gate::LSHIFT(input, factor, output);
                    self.connections.push(gate);

                    return Process::LSHIFT;
                }
                "RSHIFT" => {
                    let gate = Gate::RSHIFT(input, factor, output);
                    self.connections.push(gate);

                    return Process::RSHIFT;
                }
                _ => return Process::Error,
            }
        };

        Process::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_instruction() {
        let mut kit = WiresKit::new();

        assert!(matches!(kit.process_instruction(""), Process::Error));

        assert!(
            matches!(kit.process_instruction("123 -> x"), Process::Input),
            "Should return Process::Input"
        );
        let wire_x = kit.wire.get("x").expect("Should get wire x");
        assert!(matches!(wire_x, Wire::Signal(123)));

        assert!(
            matches!(kit.process_instruction("456 -> y"), Process::Input),
            "Should return Process::Input"
        );
        let wire_y = kit.wire.get("y").expect("Should get wire y");
        assert!(matches!(wire_y, Wire::Signal(456)));

        assert!(
            matches!(kit.process_instruction("x AND y -> d"), Process::AND),
            "Should return Process::AND"
        );
        assert!(
            matches!(kit.process_instruction("x OR y -> e"), Process::OR),
            "Should return Process::OR"
        );
        assert!(
            matches!(kit.process_instruction("x LSHIFT 2 -> f"), Process::LSHIFT),
            "Should return Process::LSHIFT"
        );
        assert!(
            matches!(kit.process_instruction("y RSHIFT 2 -> g"), Process::RSHIFT),
            "Should return Process::RSHIFT"
        );
        assert!(
            matches!(kit.process_instruction("NOT x -> h"), Process::NOT),
            "Should return Process::NOT"
        );
    }

    #[test]
    fn test_evalueate_circuit() {
        let mut kit = WiresKit::new();

        kit.process_instruction("123 -> x");
        kit.process_instruction("456 -> y");
        kit.process_instruction("x AND y -> d");
        kit.process_instruction("x OR y -> e");
        kit.process_instruction("x LSHIFT 2 -> f");
        kit.process_instruction("y RSHIFT 2 -> g");
        kit.process_instruction("NOT x -> h");
        kit.process_instruction("NOT y -> i");

        let evaluation_result = kit.evaluate_circuit();
        assert!(
            matches!(evaluation_result, Ok(())),
            "Evaluation should succed"
        );

        let wire_d = kit.wire.get("d").expect("Should get wire d");
        assert!(matches!(wire_d, Wire::Signal(72)));

        let wire_e = kit.wire.get("e").expect("Should get wire e");
        assert!(matches!(wire_e, Wire::Signal(507)));

        let wire_f = kit.wire.get("f").expect("Should get wire f");
        assert!(matches!(wire_f, Wire::Signal(492)));

        let wire_g = kit.wire.get("g").expect("Should get wire g");
        assert!(matches!(wire_g, Wire::Signal(114)));

        let wire_h = kit.wire.get("h").expect("Should get wire h");
        assert!(matches!(wire_h, Wire::Signal(65412)));

        let wire_i = kit.wire.get("i").expect("Should get wire i");
        assert!(matches!(wire_i, Wire::Signal(65079)));

        let wire_x = kit.wire.get("x").expect("Should get wire x");
        assert!(matches!(wire_x, Wire::Signal(123)));

        let wire_y = kit.wire.get("y").expect("Should get wire y");
        assert!(matches!(wire_y, Wire::Signal(456)));
    }
}
