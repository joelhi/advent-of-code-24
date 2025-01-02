use regex::Regex;

type Vec2 = [f64; 2];
type Matrix2 = [Vec2; 2];

const OFFSET: f64 = 10000000000000.;

/// Solve the problem for day 13, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    // Parse equations
    let equations = parse_equations(input_data)?;

    let result_part_1 = equations
        .iter()
        .map(|(mat, b)| solve_system(mat, b))
        .filter(is_valid)
        .map(|v| (3. * v[0] + v[1]))
        .sum::<f64>()
        .round() as u64;

    let result_part_2 = equations
        .iter()
        .map(|(mat, b)| (mat, [OFFSET + b[0], OFFSET + b[1]]))
        .map(|(mat, b)| solve_system(mat, &b))
        .filter(is_valid)
        .map(|v| (3. * v[0] + v[1]))
        .sum::<f64>()
        .round() as u64;

    Ok(vec![result_part_1, result_part_2])
}

/// Check if a solution is valid
#[inline]
fn is_valid(vec: &Vec2) -> bool {
    (vec[0] % 1.).abs() < f64::EPSILON && (vec[1] % 1.).abs() < f64::EPSILON
}

/// Parse the equations from the input data as 2x2 matrices with the constants and a solution vectors.
fn parse_equations(input_data: &[String]) -> Result<Vec<(Matrix2, Vec2)>, String> {
    let mut equations = Vec::with_capacity(input_data.len() / 4);
    let re = Regex::new(r"-?\d+").unwrap();
    for data in input_data.split(|line| line.is_empty()) {
        if data.len() == 3 {
            equations.push(read_equation_values(data, &re)?);
        }
    }

    Ok(equations)
}

/// Parse the numbers from a string
fn read_equation_values(lines: &[String], re: &Regex) -> Result<(Matrix2, Vec2), String> {
    let values: Vec<Vec<f64>> = lines
        .iter()
        .map(|line| {
            re.find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<f64>().ok())
                .collect()
        })
        .collect();

    if values.len() != 3 {
        return Err(format!("Failed to parse values from {:?}", lines));
    }

    let r1 = vec2_from_slice(values[0].as_slice())?;
    let r2 = vec2_from_slice(values[1].as_slice())?;
    let b = vec2_from_slice(values[2].as_slice())?;

    Ok((transpose_2x2(&[r1, r2]), b))
}

/// Crate rows with 2 values from a slice if possible.
fn vec2_from_slice(vals: &[f64]) -> Result<Vec2, String> {
    if vals.len() == 2 {
        Ok([vals[0], vals[1]])
    } else {
        Err("Invalid length of slice. Has to be 2.".to_owned())
    }
}

/// Transpose a 2x2 matrix.
#[inline]
fn transpose_2x2(matrix: &Matrix2) -> Matrix2 {
    [[matrix[0][0], matrix[1][0]], [matrix[0][1], matrix[1][1]]]
}

/// Solve the equation Ax=b for a system with two variables using Cramer's rule.
fn solve_system(mat: &Matrix2, b: &Vec2) -> Vec2 {
    let det = mat[0][0] * mat[1][1] - mat[1][0] * mat[0][1];
    [
        (mat[1][1] * b[0] - b[1] * mat[0][1]) / det,
        (mat[0][0] * b[1] - b[0] * mat[1][0]) / det,
    ]
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("Button A: X+94, Y+34").unwrap());
        data.push(String::from_str("Button B: X+22, Y+67").unwrap());
        data.push(String::from_str("Prize: X=8400, Y=5400").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("Button A: X+26, Y+66").unwrap());
        data.push(String::from_str("Button B: X+67, Y+21").unwrap());
        data.push(String::from_str("Prize: X=12748, Y=12176").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("Button A: X+17, Y+86").unwrap());
        data.push(String::from_str("Button B: X+84, Y+37").unwrap());
        data.push(String::from_str("Prize: X=7870, Y=6450").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("Button A: X+69, Y+23").unwrap());
        data.push(String::from_str("Button B: X+27, Y+71").unwrap());
        data.push(String::from_str("Prize: X=18641, Y=10279").unwrap());
        data.push(String::from_str("").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            480, result[0],
            "Result for part 1 example should be 480 but was {}",
            result[0]
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_13() {
        let result = solve(&crate::read_input_for_day(13).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(29201, result[0]);
        assert_eq!(104140871044942, result[1]);
    }
}
