use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn load_buffer(filename: String) -> Vec<String>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut signals: Vec<String> = Vec::new();
    for line in reader.lines(){
        signals.push(line.unwrap());
    }
    signals
}

fn find_start(signal: &String) -> usize {

    let mut p: usize = 0;
    while p < signal.len()-4 {
        let mut set: HashSet<char> = HashSet::new();
        let mut next: usize = p+1;
        while next<=p+4 {
            set.insert(signal.chars().nth(next).unwrap());
            next+=1;
        }

        if set.len() == 4 {
            return p+4+1;
        }
        p+=1;
    }
    0    
}

fn main() {
    let signals: Vec<String> = load_buffer("input_data.txt".to_string());
    for line in signals {
        let start: usize = find_start(&line);
        println!("{:} -> {:}", &line, start);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_find_start_ok() {
        assert_eq!(find_start(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(find_start(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(find_start(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 60);
        assert_eq!(find_start(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(find_start(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
}
