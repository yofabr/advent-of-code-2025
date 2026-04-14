use std::fs;

#[derive(Debug)]
struct Bank {
    grid: Vec<String>,
    max_joltage: u32,
}

impl Bank {
    fn new(grid: Vec<String>) -> Self {
        Self {
            grid: grid,
            max_joltage: 0,
        }
    }

    fn find_max_joltage(&mut self) {
        // self.max_joltage = self.grid.iter().map(|l| l.parse::<u32>().unwrap()).max().unwrap();
        let mut max = 0;

        let mut curr = 1;
        for item in &self.grid {
            if curr > self.grid.len() {
                break;
            }
            for next in curr..self.grid.len() {
                let joltage_string = format!("{item}{}", self.grid[next]);
                let joltage = joltage_string.parse::<u32>().unwrap();
                if joltage > max {
                    max = joltage;
                }
            }
            curr += 1;
        }
        self.max_joltage = max;
    }
}

fn main() {
    let inputs = fs::read_to_string("./src/input.txt").unwrap();
    println!("{}", handler(inputs));
}

fn handler(input: String) -> u32 {
    let mut output = 0;
    let mut banks: Vec<Bank> = Vec::new();

    let grids = input
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for grid in grids {
        let row = grid
            .trim()
            .split("")
            .map(|x| x.to_string())
            .filter(|x| x != &"".to_string())
            .collect::<Vec<String>>();
        let mut bank = Bank::new(row);
        bank.find_max_joltage();
        banks.push(bank);
    }
    // println!("Banks: {:?}", banks)
    //
    for bank in &banks {
        output += bank.max_joltage;
    }
    output
}
