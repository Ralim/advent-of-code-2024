use core::str;

use array2d::Array2D;

fn main() {
    let t_a = std::thread::spawn(|| {
        println!("PART A: {}", part_a("input.txt"));
    });
    let t_b = std::thread::spawn(|| {
        println!("PART B: {}", part_b("input.txt"));
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}
struct Map {
    map: Array2D<u8>,
    robot_position: (usize, usize),
}
impl Map {
    pub fn print(&self) {
        for y in 0..self.map.num_rows() {
            for x in 0..self.map.num_columns() {
                if (y, x) == self.robot_position {
                    print!("@");
                } else {
                    match self.map.get(y, x) {
                        Some(b'O') => print!("O"),
                        Some(b'#') => print!("#"),
                        Some(b'.') => print!("."),
                        Some(b'[') => print!("["),
                        Some(b']') => print!("]"),
                        _ => panic!("Invalid character"),
                    }
                }
            }
            println!();
        }
    }
    fn get_coordinate_sum(&self) -> i64 {
        self.map
            .enumerate_row_major()
            .map(|(pos, &value)| {
                if value == b'O' || value == b'[' {
                    return (pos.0 * 100) as i64 + pos.1 as i64;
                }
                0
            })
            .sum()
    }
    fn can_bump_along(&self, position: (usize, usize), direction: u8, has_recursed: bool) -> bool {
        let item_at_position = *self.map.get(position.0, position.1).unwrap();
        if !has_recursed
            && (direction == b'^' || direction == b'v')
            && (item_at_position == b'[' || item_at_position == b']')
        {
            //If we are moving up or down, and the object is a double wide box, we need to check the other half
            let offset: i64 = if item_at_position == b']' { -1 } else { 1 };
            let other_half_col = (position.1 as i64 + offset) as usize;

            if !self.can_bump_along((position.0, other_half_col), direction, true) {
                return false;
            }
        }

        //Can we bump the object at the position in the direction given
        let objects_in_way: Vec<&u8> = match direction {
            b'^' => self
                .map
                .column_iter(position.1)
                .unwrap()
                .take(position.0)
                .collect::<Vec<&u8>>()
                .into_iter()
                .rev()
                .collect(),
            b'<' => self
                .map
                .row_iter(position.0)
                .unwrap()
                .take(position.1)
                .collect::<Vec<&u8>>()
                .into_iter()
                .rev()
                .collect(),
            b'>' => self
                .map
                .row_iter(position.0)
                .unwrap()
                .skip(position.1 + 1)
                .collect(),
            b'v' => self
                .map
                .column_iter(position.1)
                .unwrap()
                .skip(position.0 + 1)
                .collect(),
            _ => panic!("Invalid instruction"),
        };
        let mut can_move = objects_in_way.contains(&&b'.');
        if !can_move {
            //Early exit
            return false;
        }
        for (index, &object) in objects_in_way.iter().enumerate() {
            if *object == b'O' {
                continue;
            }
            if *object == b'.' {
                break;
            }
            if *object == b'#' {
                can_move = false;
                break;
            }
            if *object == b'[' || *object == b']' {
                // Oh heck, its a double-wide box.
                // If we are going up or down, we need to recurse and check if the other half can move, if it cant then we are broken
                if direction == b'^' || direction == b'v' {
                    let offset: i64 = (if direction == b'^' { -1 } else { 1 }) * (index + 1) as i64;
                    //Going up down, need to check the other half
                    let other_half_position = match *object {
                        b'[' => ((position.0 as i64 + offset) as usize, position.1 + 1),
                        b']' => ((position.0 as i64 + offset) as usize, position.1 - 1),
                        _ => panic!("Invalid object"),
                    };
                    let other_can_move = self.can_bump_along(other_half_position, direction, true);
                    if !other_can_move {
                        can_move = false;
                        break;
                    }
                }
            }
        }
        can_move
    }
    fn recursively_bump_items_along(
        &mut self,
        position: (usize, usize),
        direction: u8,
        box_sync: bool,
    ) {
        //Recursively bump the items along the direction until we hit a wall or an empty space
        let current_item = *self.map.get(position.0, position.1).unwrap();
        let moving_vertically = direction == b'^' || direction == b'v';
        println!(
            "Recursively bumping {position:?} -> {} by {}",
            str::from_utf8(&[current_item]).unwrap(),
            str::from_utf8(&[direction]).unwrap()
        );
        if current_item == b'.' {
            return; //Dont move spaces, we move stuff onto them
        }
        let new_position = match direction {
            b'^' => (position.0 - 1, position.1),
            b'<' => (position.0, position.1 - 1),
            b'>' => (position.0, position.1 + 1),
            b'v' => (position.0 + 1, position.1),
            _ => panic!("Invalid instruction `{direction}`"),
        };
        let mut new_item = *self.map.get(new_position.0, new_position.1).unwrap();
        if new_item == b'#' {
            self.print();
            panic!("Cant move walls {box_sync}");
        }
        if new_item != b'.' {
            // The space is taken, we need to recursively move the item at the new position in the same direction

            //If we are moving up or down and the box is double wide `[]` we need to move the other half as well

            if moving_vertically && (new_item == b'[' || new_item == b']') {
                println!("Vertical, double box, case");
                let other_half_position = match new_item {
                    b'[' => (new_position.0, new_position.1 + 1),
                    b']' => (new_position.0, new_position.1 - 1),
                    _ => panic!("Invalid object"),
                };
                self.recursively_bump_items_along(other_half_position, direction, true);
            }
            self.recursively_bump_items_along(new_position, direction, true);
        }
        // Special case, if we are moving up or down and the box is double wide `[]` we need to move the other half as well
        if moving_vertically && (current_item == b'[' || current_item == b']') && !box_sync {
            println!("Vertical, double box, case");
            let other_half_position = match current_item {
                b'[' => (position.0, position.1 + 1),
                b']' => (position.0, position.1 - 1),
                _ => panic!("Invalid object"),
            };
            self.recursively_bump_items_along(other_half_position, direction, true);
        }
        //Grab the new item, just to check its a '.'
        new_item = *self.map.get(new_position.0, new_position.1).unwrap();
        if new_item == b'#' {
            panic!("Cant move walls");
        }
        if new_item == b'.' {
            //Move the item to the new position
            self.map
                .set(new_position.0, new_position.1, current_item)
                .unwrap();
            self.map.set(position.0, position.1, new_item).unwrap();
        } else {
            self.print();
            panic!("How did we get here??");
        }
    }
    fn apply_instruction(&mut self, instruction: u8) {
        let new_position = match instruction {
            b'^' => (self.robot_position.0 - 1, self.robot_position.1),
            b'<' => (self.robot_position.0, self.robot_position.1 - 1),
            b'>' => (self.robot_position.0, self.robot_position.1 + 1),
            b'v' => (self.robot_position.0 + 1, self.robot_position.1),
            _ => panic!("Invalid instruction `{instruction}`"),
        };
        //Check if the new position is valid, if its going to move it onto an object 'O', we bump objects if possible
        let item = self.map.get(new_position.0, new_position.1).unwrap();
        if *item == b'.' {
            //Trivial case, it just moves onto new position
            self.robot_position = new_position;
            // println!("Empty,trivial op");
            return;
        }
        if *item == b'#' {
            //It hits a wall, do nothing
            // println!("Wall, no-op");
            return;
        }
        // Only case left is the boxes

        // Need to check if we need to move the object
        // We can bump the objects if from the current robot position to the wall has at least 1 empty space, in the direction of travel
        let can_move = self.can_bump_along(new_position, instruction, false);
        println!("Robot can move? {can_move}");
        if !can_move {
            return;
        }
        self.recursively_bump_items_along(new_position, instruction, false);

        self.robot_position = new_position;
    }
}
fn load_file(path: &str, doubler: bool) -> (Map, Vec<u8>) {
    let mut file_contents = std::fs::read_to_string(path).unwrap();
    if doubler {
        file_contents = file_contents
            .replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.");
    }
    let lines: Vec<&str> = file_contents.lines().collect();
    // lines.iter().for_each(|l| println!("Line> {l}"));

    let split_blank_line_inded = lines.iter().position(|&x| x.is_empty()).unwrap();
    let map_lines = &lines[..split_blank_line_inded];
    let instruction_data = &lines[split_blank_line_inded + 1..]
        .concat()
        .replace("\r", "")
        .replace("\n", "");

    let mut map = Array2D::from_row_major(
        map_lines.concat().as_bytes(),
        map_lines.len(),
        map_lines[0].len(),
    )
    .unwrap();
    let robot_position = map
        .enumerate_row_major()
        .filter_map(|(pos, &value)| if value == b'@' { Some(pos) } else { None })
        .next()
        .unwrap();
    //Replace the robot position with a dot to clear it
    map.set(robot_position.0, robot_position.1, b'.').unwrap();
    (
        Map {
            map,
            robot_position,
        },
        instruction_data.trim().as_bytes().to_vec(),
    )
}
fn part_a(path: &str) -> i64 {
    let (mut map, instructions) = load_file(path, false);
    map.print();
    for instruction in instructions {
        map.apply_instruction(instruction);
        // map.print();
    }
    map.get_coordinate_sum()
}
fn part_b(path: &str) -> i64 {
    let (mut map, instructions) = load_file(path, true);
    map.print();
    for instruction in instructions {
        map.apply_instruction(instruction);
        // map.print();
    }
    map.get_coordinate_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 10092);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 1499739);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 9021);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 1522215);
    }
}
