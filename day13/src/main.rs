use nalgebra::{Matrix2, Vector2};
use rayon::prelude::*;
use regex::Regex;

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
#[derive(Debug, Default)]
struct Machine {
    button_a_increment: (i64, i64),
    button_b_increment: (i64, i64),
    target_location: (i64, i64),
}
impl Machine {
    pub fn find_cost(&self, button_a_cost: i64, button_b_cost: i64) -> Option<i64> {
        // Solving simultaneous equations of
        // Z_x = a_x*X + b_x*Y && Z_y = a_y*X + b_y*Y

        /*
         A & B are button counts
         We get
         a_x*A + b_x*B = Z_x
         a_y*A + b_y*B = Z_y
         A  = (Z_y -  b_y*B)/a_y
         a_x*((Z_y -  b_y*B)/a_y) + b_x*B = Z_x


         These can be written to the matix as

         | a_x b_x | | A | = | Z_x |
         | a_y b_y | | B | = | Z_y |

         Therefore can re-write this into A*X=B, where A,X,B are each matrix above
         If A*X=B; X = A^-1 * B
        We have two unknowns, and two equations; therefore there is always one solution


         */
        let knowns = Vector2::new(self.target_location.0 as f64, self.target_location.1 as f64);

        let unknowns = Matrix2::new(
            self.button_a_increment.0 as f64,
            self.button_b_increment.0 as f64,
            self.button_a_increment.1 as f64,
            self.button_b_increment.1 as f64,
        );
        let unknowns_inverse = unknowns.try_inverse()?;

        let solution = unknowns_inverse * knowns;
        let a_count = solution[0].round();
        let b_count = solution[1].round();
        // println!("Possible solution {self:?}-> {a_count} {b_count}");
        //Confirm its a real-number solution
        let check_position = (
            a_count * self.button_a_increment.0 as f64 + b_count * self.button_b_increment.0 as f64,
            a_count * self.button_a_increment.1 as f64 + b_count * self.button_b_increment.1 as f64,
        );
        let check_pos_i64 = (
            check_position.0.round() as i64,
            check_position.1.round() as i64,
        );

        if check_pos_i64 != self.target_location {
            // println!(
            //     "Checking {self:?}->  {:?} {:?} {:?}",
            //     check_position, solution, check_pos_i64
            // );
            return None;
        }
        Some(a_count as i64 * button_a_cost + b_count as i64 * button_b_cost)
    }
}
fn machines_from_input(file: &str) -> Vec<Machine> {
    let mut machines = Vec::with_capacity(100);
    let input = std::fs::read_to_string(file).unwrap();
    let mut current_machine = Machine::default();
    for line in input.lines() {
        if line.is_empty() {
            //End of machine
            machines.push(current_machine);
            current_machine = Machine::default();
        } else {
            let regex_pos: Regex = Regex::new(r"X[+=](\d+), Y[+=](\d+)$").unwrap();
            let (x, y) = regex_pos
                .captures(line)
                .map(|cap| {
                    let x = cap[1].parse::<i64>().unwrap();
                    let y = cap[2].parse::<i64>().unwrap();
                    (x, y)
                })
                .unwrap();
            match &line[0..8] {
                "Button A" => current_machine.button_a_increment = (x, y),
                "Button B" => current_machine.button_b_increment = (x, y),
                "Prize: X" => current_machine.target_location = (x, y),
                _ => panic!("Unknown line: {}", line),
            }
        }
    }
    machines.push(current_machine);
    machines
}
fn part_a(path: &str) -> i64 {
    let machines = machines_from_input(path);
    // println!("Machines {:?}", machines);

    machines
        .par_iter()
        .map(|machine| {
            let cost = machine.find_cost(3, 1);
            // println!("Cost: {:?} -> {:?}", machine, cost);
            cost.unwrap_or_default()
        })
        .sum()
}
fn part_b(path: &str) -> i64 {
    let mut machines = machines_from_input(path);
    // println!("Machines {:?}", machines);
    //Update machine targers
    machines.iter_mut().for_each(|machine| {
        machine.target_location.0 += 10000000000000;
        machine.target_location.1 += 10000000000000;
    });

    machines
        .into_iter()
        .map(|machine| machine.find_cost(3, 1).unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 480);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 28262);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 875318608908);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 101406661266314);
    }
}
