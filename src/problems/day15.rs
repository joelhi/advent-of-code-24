use hashbrown::HashSet;

use crate::{increment_2d_index, Vec2i, Vec2u};

type MapData = (HashSet<Vec2u>, HashSet<Vec2u>);

/// Solve the problem for day 15, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (mut map_data, start, commands) = parse_input(input_data)?;

    let mut expanded = expand_map_data(&map_data);
    // Part 1
    execute_commands(&mut map_data, start, commands, false)?;

    let result_part_1 = map_data
        .1
        .iter()
        .map(|pos| 100 * pos.0 + pos.1)
        .sum::<usize>() as u64;

    // Part 2
    execute_commands(&mut expanded, (start.0, start.1 * 2), commands, true)?;

    let result_part_2 = expanded
        .1
        .iter()
        .map(|pos| 100 * pos.0 + pos.1)
        .sum::<usize>() as u64;

    Ok(vec![result_part_1, result_part_2])
}

fn expand_map_data(map_data: &MapData) -> MapData {
    let expanded_walls = map_data.0.iter().map(|&pos| (pos.0, 2 * pos.1)).collect();
    let expanded_boxes = map_data.1.iter().map(|&pos| (pos.0, 2 * pos.1)).collect();

    (expanded_walls, expanded_boxes)
}

/// Execute a sequence of commands and move the boxes.
fn execute_commands(
    map_data: &mut MapData,
    start: Vec2u,
    commands: &[String],
    expanded: bool,
) -> Result<(), String> {
    let mut pos = start;
    for sequence in commands {
        for command in sequence.chars() {
            execute_command(map_data, &mut pos, &command, expanded)?;
        }
    }

    Ok(())
}

/// Execute a command, and update the current position and boxes if applicable.
fn execute_command(
    map_data: &mut MapData,
    pos: &mut Vec2u,
    command: &char,
    expanded: bool,
) -> Result<(), String> {
    let (walls, boxes) = map_data;
    let step = step_from_command(command)?;
    let new_pos = increment_2d_index(pos.0, pos.1, step.0, step.1, 1).ok_or(format!(
        "Failed to increment coord {:?} with {:?}",
        pos, step
    ))?;

    if hits_wall(walls, &new_pos, expanded) {
        return Ok(());
    } else if let Some(collision_box) = overlaps_box(boxes, &new_pos, expanded) {
        let mut to_shift = HashSet::new();
        to_shift.insert(collision_box);
        if can_move_box(boxes, walls, &collision_box, &step, &mut to_shift, expanded)? {
            update_boxes(boxes, &to_shift, &step);
            *pos = new_pos;
        }
    } else {
        *pos = new_pos;
    }

    Ok(())
}

/// Update the positions of the shifted boxes
fn update_boxes(boxes: &mut HashSet<Vec2u>, to_shift: &HashSet<Vec2u>, step: &Vec2i) {
    for pos in to_shift.iter() {
        boxes.remove(pos);
    }
    for pos in to_shift.iter() {
        if let Some(new_pos) = increment_2d_index(pos.0, pos.1, step.0, step.1, 1) {
            boxes.insert(new_pos);
        }
    }
}

/// Move box to next position if possible
fn can_move_box(
    boxes: &mut HashSet<Vec2u>,
    walls: &HashSet<Vec2u>,
    pos: &Vec2u,
    step: &Vec2i,
    to_shift: &mut HashSet<Vec2u>,
    expanded: bool,
) -> Result<bool, String> {
    let next_pos = search_positions(pos, step, expanded)?;
    for new_pos in next_pos.iter().filter_map(|p| p.map(|p| p)) {
        if hits_wall(walls, &new_pos, expanded) {
            return Ok(false);
        }
        if let Some(collision) = overlaps_box(boxes, &new_pos, expanded) {
            if to_shift.insert(collision)
                && !can_move_box(boxes, walls, &collision, step, to_shift, expanded)?
            {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// Check if a certain location contains a wall char.
fn hits_wall(walls: &HashSet<Vec2u>, pos: &Vec2u, expanded: bool) -> bool {
    walls.contains(pos) || (expanded && walls.contains(&(pos.0, pos.1 - 1)))
}

/// Returns the next set of search positions for validating box move.
fn search_positions(
    pos: &Vec2u,
    step: &Vec2i,
    expanded: bool,
) -> Result<[Option<Vec2u>; 2], String> {
    fn try_increment(
        pos: &Vec2u,
        step_i: isize,
        step_j: isize,
        scale: usize,
    ) -> Result<Vec2u, String> {
        increment_2d_index(pos.0, pos.1, step_i, step_j, scale).ok_or_else(|| {
            format!(
                "Failed to increment coord {:?} with step_x: {}, step_y: {}, scale: {}",
                pos, step_i, step_j, scale
            )
        })
    }

    let next_pos = try_increment(pos, step.0, step.1, 1)?;

    let next_pos_2 = if expanded {
        if step.0 != 0 {
            Some(try_increment(pos, step.0, step.1 + 1, 1)?)
        } else if step.1 > 0 {
            Some(try_increment(pos, step.0, step.1, 2)?)
        } else {
            None
        }
    } else {
        None
    };

    // Return the results
    Ok([Some(next_pos), next_pos_2])
}

/// Returns a coordinate for a box if it overlaps, otherwise [`None`]
fn overlaps_box(boxes: &HashSet<Vec2u>, pos: &Vec2u, expanded: bool) -> Option<Vec2u> {
    if boxes.contains(pos) {
        return Some(*pos);
    } else if expanded && boxes.contains(&(pos.0, pos.1 - 1)) {
        return Some((pos.0, pos.1 - 1));
    }

    None
}

/// Get the step vector based on the command char.
fn step_from_command(command: &char) -> Result<Vec2i, String> {
    match command {
        '<' => Ok((0, -1)),
        '^' => Ok((-1, 0)),
        'v' => Ok((1, 0)),
        '>' => Ok((0, 1)),
        _ => Err(format!("Invalid command character {}", command)),
    }
}

/// Parse the input data
fn parse_input(input_data: &[String]) -> Result<(MapData, Vec2u, &[String]), String> {
    let mut split = input_data.split(|line| line.is_empty());

    let (map_data, start) = parse_input_map(
        split
            .next()
            .ok_or("Failed to parse map from input".to_owned())?,
    );

    let commands = split
        .next()
        .ok_or("Failed to parse map from input".to_owned())?;

    Ok((map_data, start, commands))
}

/// Parse the locations of all boxes and walls into two hash sets (walls, boxes).
fn parse_input_map(map: &[String]) -> (MapData, Vec2u) {
    let mut wall_pos = HashSet::new();
    let mut box_pos = HashSet::new();
    let mut start = (1, 1);
    for (i, line) in map.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    wall_pos.insert((i, j));
                }
                'O' => {
                    box_pos.insert((i, j));
                }
                '@' => {
                    start = (i, j);
                }
                _ => (),
            }
        }
    }

    ((wall_pos, box_pos), start)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_small_example_data_part_1() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("########").unwrap());
        data.push(String::from_str("#..O.O.#").unwrap());
        data.push(String::from_str("##@.O..#").unwrap());
        data.push(String::from_str("#...O..#").unwrap());
        data.push(String::from_str("#.#.O..#").unwrap());
        data.push(String::from_str("#...O..#").unwrap());
        data.push(String::from_str("#......#").unwrap());
        data.push(String::from_str("########").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("<^^>>>vv<v>>v<<").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            2028, result[0],
            "Result for part 1 example should be 2028 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_large_example_data_part_2() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("##########").unwrap());
        data.push(String::from_str("#..O..O.O#").unwrap());
        data.push(String::from_str("#......O.#").unwrap());
        data.push(String::from_str("#.OO..O.O#").unwrap());
        data.push(String::from_str("#..O@..O.#").unwrap());
        data.push(String::from_str("#O#..O...#").unwrap());
        data.push(String::from_str("#O..O..O.#").unwrap());
        data.push(String::from_str("#.OO.O.OO#").unwrap());
        data.push(String::from_str("#....O...#").unwrap());
        data.push(String::from_str("##########").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(
            String::from_str(
                "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
            )
            .unwrap(),
        );
        data.push(
            String::from_str(
                "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            )
            .unwrap(),
        );

        let result = solve(&data).unwrap();
        assert_eq!(
            9021, result[1],
            "Result for part 2 example should be 9021 but was {}",
            result[1]
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_15() {
        let result = solve(&crate::read_input_for_day(15).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(1505963, result[0]);
        assert_eq!(1543141, result[1]);
    }
}
