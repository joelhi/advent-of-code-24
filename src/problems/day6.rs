use std::collections::HashSet;

use super::utils;

type GuardData = (usize, usize, isize, isize);

/// Safety switch for the while loop
const MAX_STEPS: usize = 100000;

/// Solve the problem for day six, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    let result_part_1 = solve_part_1(input_data)?;

    Ok(vec![result_part_1, 0])
}

/// Solve part 1
fn solve_part_1(input_data: &[String]) -> Result<u32, String> {
    let mut guard_data= Some(find_guard_loc_and_dir(input_data)?);
    let mut visited = HashSet::new();
    let mut breaker = 0;
    while let Some(valid_guard) = guard_data {
        visited.insert((valid_guard.0, valid_guard.1));
        println!("{}:{},{}", breaker, valid_guard.0, valid_guard.1);
        guard_data = update_loc_and_dir(valid_guard, input_data)?;
        breaker+=1;
        if breaker == MAX_STEPS{
            break;
        }
    }

    Ok(visited.len() as u32)
}

/// Draw the visited position
fn trace(input_data: &[String], path: &HashSet<(usize, usize)>){
    let mut data_copy = input_data.iter().map(|s| s.clone()).collect::<Vec<String>>();

    for (i, j) in path{
        data_copy[*i].replace_range(*j..*j+1, "x");
    }

    for line in data_copy{
        println!("{}", line);
    }
}

/// Update the position and direction based on the guards movement.
fn update_loc_and_dir(guard_data: GuardData, input_data: &[String]) -> Result<Option<GuardData>, String> {
    let (i, j, v_i, v_j) = guard_data;
    if let Some((i_new, j_new)) =
        utils::increment_2d_index(guard_data.0, guard_data.1, guard_data.2, guard_data.3, 1)
    {
        if let Some(hit) = has_hit_obstacle(i_new, j_new, input_data){
            if hit{
                // Hits obstacle, try again from original spot with updated direction
                let (v_i, v_j) = rotate_dir(v_i, v_j)?;
                return update_loc_and_dir((i, j, v_i, v_j), input_data);
            }else{
                // Clear path
                return Ok(Some((i_new, j_new, v_i, v_j)));
            }
        }else{
            // No valid position at new, overflow
            return Ok(None)
        }
    }
    
    // Underflow, no valid position
    Ok(None)
}

/// Rotate the direction clockwise
fn rotate_dir(v_i: isize, v_j: isize)->Result<(isize, isize), String>{
    if v_i < 0{
        return Ok((0, 1));
    }else if v_i > 0{
        return Ok((0, -1));
    }else if v_j < 0 {
        return Ok((-1, 0));
    }else if v_j > 0 {
        return Ok((1, 0));
    }

    Err(format!("Cannot rotate direction {},{} as it's not valid.", v_i, v_j))
}

/// Check if the current position has an obstacle
fn has_hit_obstacle(i: usize, j: usize, input_data: &[String])->Option<bool>{
    if let Some(char) = utils::get_char(input_data, i, j) {
        Some(char == '#')
    }
    else{
        None
    }
}

/// Find the location and direction of travel for the guard in the data.
fn find_guard_loc_and_dir(input_data: &[String]) -> Result<GuardData, String> {
    for (i, s) in input_data.iter().enumerate() {
        if let Some(j) = s.find(">") {
            return Ok((i, j, 0, 1));
        } else if let Some(j) = s.find("^") {
            return Ok((i, j, -1, 0));
        } else if let Some(j) = s.find("<") {
            return Ok((i, j, 0, -1));
        } else if let Some(j) = s.find("v") {
            return Ok((i, j, -1, 0));
        }
    }
    Err("Not valid guard found.".to_owned())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::solve;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("....#.....").unwrap());
        data.push(String::from_str(".........#").unwrap());
        data.push(String::from_str("..........").unwrap());
        data.push(String::from_str("..#.......").unwrap());
        data.push(String::from_str(".......#..").unwrap());
        data.push(String::from_str("..........").unwrap());
        data.push(String::from_str(".#..^.....").unwrap());
        data.push(String::from_str("........#.").unwrap());
        data.push(String::from_str("#.........").unwrap());
        data.push(String::from_str("......#...").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            41, result[0],
            "Result for part 1 example should be 18 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_6() {
        let result =
            solve(&read_input_for_day(&6).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(5080, result[0]);
        assert_eq!(0, result[1]);
    }
}










