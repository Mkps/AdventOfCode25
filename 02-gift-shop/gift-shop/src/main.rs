fn is_repeated_pattern(s: &str) -> bool {
    if s.len() <= 1 {
        return false;
    }

    let doubled = format!("{s}{s}");
    let inner = &doubled[1..doubled.len() - 1];
    inner.contains(s)
}

fn check_these_repeating_digits(start: u64, end: u64) -> u64 {
    let mut ret = 0;

    for n in start..=end {
        let s = n.to_string();
        if is_repeated_pattern(&s){
            ret += n;
        }
    }
    // println!("Ret is {}", ret);
    ret
}

fn challenge2() {
    let raw = std::fs::read_to_string("example.txt");
    let mut answer: u64 = 0;
    let mut input = raw.expect("Could not split input.txt");
    input = input.trim().to_string(); //Triming ending newline or we get a panic
    let values = input.split(",");
    for pair in values {
        // println!("Value is {}", pair);
        let mut iter = pair.split("-");
        let a = iter.next().unwrap().parse::<u64>().unwrap();
        let b = iter.next().unwrap().parse::<u64>().unwrap();
        answer += check_these_repeating_digits(a, b);
    }
    println!("Answer is {}", answer);
}

fn check_these_dubs(start: u64, end: u64) -> u64 {
    let mut ret = 0;

    for n in start..=end {
        let s = n.to_string();

        if s.len() % 2 == 0 {
            let half = s.len() / 2;
            let (left, right) = s.split_at(half);

            if left == right {
                ret += n;
            }
        }
    }
    // println!("Ret is {}", ret);
    ret
}

fn challenge1() {
    let raw = std::fs::read_to_string("input.txt");
    let mut answer: u64 = 0;
    let mut input = raw.expect("Could not split input.txt");
    input = input.trim().to_string(); //Triming ending newline or we get a panic
    let values = input.split(",");
    for pair in values {
        // println!("Value is {}", pair);
        let mut iter = pair.split("-");
        let a = iter.next().unwrap().parse::<u64>().unwrap();
        let b = iter.next().unwrap().parse::<u64>().unwrap();
        answer += check_these_dubs(a, b);
    }
    println!("Answer is {}", answer);
}

fn main() {

    challenge1();
    challenge2();
}
