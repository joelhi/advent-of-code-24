use regex::Regex;

type Registers = (u64, u64, u64);

/// Solve the problem for day 16, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (mut registers, program) = parse_input(input_data)?;

    let output = compute_program(&mut registers, &program)?;

    Ok(vec![
        output
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap(),
        0,
    ])
}

fn compute_program(registers: &mut Registers, program: &[u64]) -> Result<Vec<u64>, String> {
    let mut output = Vec::new();
    let mut i = 0;
    let mut increment = true;
    loop {
        println!(
            "Exectuting instruction {}, op: {} operand: {}",
            i,
            program[i],
            program[i + 1]
        );
        if let Some(val) = compute_operation(
            program[i],
            program[i + 1],
            registers,
            &mut i,
            &mut increment,
        )? {
            output.push(val);
        }

        if increment {
            i += 2;
        } else {
            increment = true;
        }

        if i >= program.len() {
            break;
        }
    }

    Ok(output)
}

/// Compute the given operation with the given operand.
fn compute_operation(
    operation: u64,
    operand: u64,
    registers: &mut Registers,
    i: &mut usize,
    increment: &mut bool,
) -> Result<Option<u64>, String> {
    match operation {
        0 => {
            registers.0 = registers.0 / (2_u64.pow(combo_operand(operand, registers)? as u32));
        }
        1 => {
            registers.1 = registers.1 ^ operand;
        }
        2 => {
            registers.1 = combo_operand(operand, registers)? % 8;
        }
        3 => {
            if registers.0 != 0 {
                *i = operand as usize;
                *increment = false;
            }
        }
        4 => {
            registers.1 = registers.1 ^ registers.2;
        }
        5 => {
            return Ok(Some(combo_operand(operand, registers)? % 8));
        }
        6 => {
            registers.1 = registers.0 / (2_u64.pow(combo_operand(operand, registers)? as u32));
        }
        7 => {
            registers.2 = registers.0 / (2_u64.pow(combo_operand(operand, registers)? as u32));
        }
        _ => return Err(format!("Invalid operation: {}", operation)),
    };

    Ok(None)
}

/// Get the combo operand for the value
fn combo_operand(operand: u64, registers: &Registers) -> Result<u64, String> {
    match operand {
        0..=3 => Ok(operand),
        4 => Ok(registers.0),
        5 => Ok(registers.1),
        6 => Ok(registers.2),
        _ => return Err(format!("No valid combo operand for: {}", operand)),
    }
}

/// Parse the registers and program commands
fn parse_input(input_data: &[String]) -> Result<(Registers, Vec<u64>), String> {
    let mut split = input_data.split(|s| s.is_empty());
    let registers = split.next().ok_or("Failed to parse inputs")?;
    let program = &split.next().ok_or("Failed to parse inputs")?[0];

    Ok((parse_registers(registers)?, parse_program(program)?))
}

/// Read the inital values from the input for the registers
fn parse_registers(input_data: &[String]) -> Result<Registers, String> {
    let re = Regex::new(r"-?\d+").map_err(|_| "Failed to compile regex.")?;
    if input_data.len() != 3 {
        return Err(format!(
            "Invalid slice length. Expected 3 but was {}",
            input_data.len()
        ));
    }

    let reg_a = re
        .find(&input_data[0])
        .ok_or(format!("Failed to read number from {}", input_data[0]))?
        .as_str()
        .parse::<u64>()
        .map_err(|_| format!("Failed to read number from {}", input_data[0]))?;

    let reg_b = re
        .find(&input_data[1])
        .ok_or(format!("Failed to read number from {}", input_data[1]))?
        .as_str()
        .parse::<u64>()
        .map_err(|_| format!("Failed to read number from {}", input_data[1]))?;

    let reg_c = re
        .find(&input_data[2])
        .ok_or(format!("Failed to read number from {}", input_data[2]))?
        .as_str()
        .parse::<u64>()
        .map_err(|_| format!("Failed to read number from {}", input_data[2]))?;

    Ok((reg_a, reg_b, reg_c))
}

/// Parse the program sequence
fn parse_program(input: &str) -> Result<Vec<u64>, String> {
    let re = Regex::new(r"-?\d+").map_err(|_| "Failed to compile regex.")?;

    let nums: Vec<u64> = re
        .find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<u64>().ok())
        .collect();

    Ok(nums)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let map = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve(&data).unwrap();
        assert_eq!(
            4635635210, result[0],
            "Result for part 1 example should be 4635635210 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_17() {
        let result =
            solve(&read_input_for_day(17).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(150373031, result[0]);
        assert_eq!(0, result[1]);
    }
}
