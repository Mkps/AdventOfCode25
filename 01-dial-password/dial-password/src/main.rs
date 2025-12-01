fn main() {
    let mut current_position = 50;
    let input = std::fs::read_to_string("input.txt");
    let mut password_count = 0;
    for line in input.expect("REASON").lines() {
        let dir = line.chars().next().unwrap();
        let mut num: i32 = line[1..].parse().unwrap();
        // println!("{}", dir);
        // println!("{}", num);
        if dir == 'L' {
            num *= -1;
        } 
        current_position = (current_position + num % 100 + 100) % 100;
        if current_position == 0 {
            password_count += 1;
        }
    }
    println!("Password count is : {}", password_count)
}
