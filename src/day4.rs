use crate::input;

pub fn day4() -> input::Result<()> {
    let contents = input::load_day_file("day4.txt");

    let mut result: u32 = 0;

    for line in contents.lines() {
        // split the line on the , and store the result in a vector
        let split_line: Vec<&str> = line.split(",").collect();
        let first_elf: Vec<u32> = split_line[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let second_elf: Vec<u32> = split_line[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect();

        if first_elf[0] >= second_elf[0] && first_elf[1] <= second_elf[1] {
            result += 1;
        }
        else if first_elf[0] <= second_elf[0] && first_elf[1] >= second_elf[1] {
            result += 1;
        }
        // step 2
        else if first_elf[1] >= second_elf[0] && first_elf[1] <= second_elf[1] {
            result += 1;
        }
        else if first_elf[0] >= second_elf[0] && first_elf[0] <= second_elf[1] {
            result += 1;
        }
    }

        println!("Result: {}", result);
    

    Ok(())
}
