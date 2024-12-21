use regex::Regex;

use crate::checked_add_signed_increment;

type Vec2u = (usize, usize);
type Vec2i = (isize, isize);

/// Solve the problem for day 14, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    solve_for_map_size(input_data, (101, 103))
}

/// Solve for a certain set of robots and size of the map.
fn solve_for_map_size(input_data: &[String], map_size: Vec2i) -> Result<Vec<u64>, String> {
    let robot_data = parse_robot_data(input_data)?;

    // Part 1
    let moved_robots = move_robots(&robot_data, map_size, 100)?;
    let result_part_1 = count_quadrants(&moved_robots, map_size.0 as usize, map_size.1 as usize);

    // Part 2
    let mut min_sd = f64::MAX;
    let mut min_i = 0;
    for i in 0..10000 {
        let update = move_robots(&robot_data, map_size, i)?;
        let sd = std_dev(&update);
        if sd < min_sd {
            min_sd = sd;
            min_i = i;
        }
    }

    Ok(vec![result_part_1, min_i as u64])
}

/// Compute the 2D standard deviation about the mean position of the data.
fn std_dev(robot_data: &[(Vec2u, Vec2i)]) -> f64 {
    let num_coords = robot_data.len() as f64;

    let (x_sum, y_sum) = robot_data
        .iter()
        .fold((0.0, 0.0), |(x_acc, y_acc), (pos, _)| {
            (x_acc + pos.0 as f64, y_acc + pos.1 as f64)
        });
    let mean = (x_sum / num_coords, y_sum / num_coords);

    let sum_squared_distances = robot_data
        .iter()
        .map(|(pos, _)| {
            let x = pos.0 as f64;
            let y = pos.1 as f64;
            let dx = x - mean.0;
            let dy = y - mean.1;
            dx.powi(2) + dy.powi(2)
        })
        .sum::<f64>();

    (sum_squared_distances / num_coords).sqrt()
}

/// Count the number in each quadrant and multiply the results
fn count_quadrants(robot_data: &[(Vec2u, Vec2i)], w: usize, h: usize) -> u64 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in robot_data.iter() {
        if r.0 .0 < w / 2 && r.0 .1 < h / 2 {
            q1 += 1;
        } else if r.0 .0 > w / 2 && r.0 .1 < h / 2 {
            q2 += 1;
        } else if r.0 .0 < w / 2 && r.0 .1 > h / 2 {
            q3 += 1;
        } else if r.0 .0 > w / 2 && r.0 .1 > h / 2 {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

/// Parse the data for each robot, as (position, velocity).
fn parse_robot_data(input_data: &[String]) -> Result<Vec<(Vec2u, Vec2i)>, String> {
    let re = Regex::new(r"-?\d+").map_err(|_| "Failed to compile regex.")?;

    input_data
        .iter()
        .enumerate()
        .map(|(line_id, line)| parse_line(line, line_id, &re))
        .collect()
}

/// Parse the values from a lines into a position and a velicity tuple.
fn parse_line(line: &str, line_id: usize, re: &Regex) -> Result<(Vec2u, Vec2i), String> {
    let nums: Vec<isize> = re
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<isize>().ok())
        .collect();

    if nums.len() != 4 {
        return Err(format!(
            "Line {}: Expected 4 numbers, but found {}. Line content: '{}'",
            line_id,
            nums.len(),
            line
        ));
    }

    let pos = (
        nums[0]
            .try_into()
            .map_err(|_| format!("Line {}: Position x must be non-negative.", line_id))?,
        nums[1]
            .try_into()
            .map_err(|_| format!("Line {}: Position y must be non-negative.", line_id))?,
    );

    Ok((pos, (nums[2], nums[3])))
}

/// Move the robots based on their positions, speed and number of seconds.
fn move_robots(
    robot_data: &[(Vec2u, Vec2i)],
    map_size: Vec2i,
    seconds: usize,
) -> Result<Vec<(Vec2u, Vec2i)>, String> {
    robot_data
        .iter()
        .map(|(r_pos, r_v)| {
            let i = checked_add_signed_increment(r_pos.0, r_v.0, seconds as isize).ok_or(
                format!("Failed to increment position {} with {}.", r_pos.0, r_v.0),
            )?;
            let j = checked_add_signed_increment(r_pos.1, r_v.1, seconds as isize).ok_or(
                format!("Failed to increment position {} with {}.", r_pos.1, r_v.1),
            )?;
            let mut mod_i = i % (map_size.0);
            let mut mod_j = j % (map_size.1);

            if mod_i < 0 {
                mod_i += map_size.0;
            }
            if mod_j < 0 {
                mod_j += map_size.1;
            }

            Ok(((mod_i as usize, mod_j as usize), *r_v))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("p=0,4 v=3,-3").unwrap());
        data.push(String::from_str("p=6,3 v=-1,-3").unwrap());
        data.push(String::from_str("p=10,3 v=-1,2").unwrap());
        data.push(String::from_str("p=2,0 v=2,-1").unwrap());
        data.push(String::from_str("p=0,0 v=1,3").unwrap());
        data.push(String::from_str("p=3,0 v=-2,-2").unwrap());
        data.push(String::from_str("p=7,6 v=-1,-3").unwrap());
        data.push(String::from_str("p=3,0 v=-1,-2").unwrap());
        data.push(String::from_str("p=9,3 v=2,3").unwrap());
        data.push(String::from_str("p=7,3 v=-1,2").unwrap());
        data.push(String::from_str("p=2,4 v=2,-3").unwrap());
        data.push(String::from_str("p=9,5 v=-3,-3").unwrap());

        let result = solve_for_map_size(&data, (11, 7)).unwrap();
        assert_eq!(
            12, result[0],
            "Result for part 1 example should be 12 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_14() {
        let result =
            solve(&read_input_for_day(14).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(232253028, result[0]);
        assert_eq!(8179, result[1]);
    }
}
