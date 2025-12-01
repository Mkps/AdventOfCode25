fn main() {
    let mut current_position: i32 = 50;
    let input = std::fs::read_to_string("input.txt");
    let mut password_count: u32 = 0;

    for line in input.expect("Could not open input.txt").lines() {
        let dir = line.chars().next().unwrap();
        let num: i32 = line[1..].parse().unwrap();

        let rot = if dir == 'L' { 
            -num
        } else {
            num
        };

        let sum = current_position + rot;

        // If we actually looped from the left
        if dir == 'L'  && sum <= 0{
            // Avoid double counting when we're already on 0
            password_count += ((sum / 100).abs() + 1 - (current_position == 0) as i32) as u32;
        }
        // Or from the right
        else if sum >= 100 {
            password_count += (sum / 100) as u32;
        } 
        current_position = (current_position + rot % 100 + 100) % 100;
    }
    println!("Number of time we hit 0: {} times.", password_count)
}
