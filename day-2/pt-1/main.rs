fn main() {
    let input = std::fs::read_to_string("./src/input.txt").unwrap();

    let out = handle(input);
    // let out = get_invalid_pairs(input);
    println!("{out}")
}

fn handle(input: String) -> i64 {
    let mut output = 0;
    let ranges: Vec<String> = input.trim().split(",").map(|x| x.to_string()).collect();

    for range in ranges {
        // println!("Range: {range}");
        output += get_invalid_pairs(range);
    }
    output
}

fn get_invalid_pairs(range: String) -> i64 {
    let mut output = 0;

    let nums = range
        .split("-")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // println!("{:?}", nums);
    for num in nums[0]..=nums[1] {
        // println!("{}", num);
        let invalid = find_pattern(num);
        if invalid {
            // println!("Invalid: {num}");
            output += num;
        }
    }
    output
}
// 1 2 3 1123112311231
fn find_pattern(inp: i64) -> bool {
    if inp.to_string().len() < 2 {
        return false;
    }
    let stringified = inp.to_string();
    let half = stringified.len() / 2;
    let first_half = &stringified[..half];
    let second_half = &stringified[half..];

    if first_half == second_half {
        return true;
    }
    false
}

//121212
