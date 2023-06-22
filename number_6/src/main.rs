use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn load_buffer(filename: String) -> Vec<String>{

    let hfile = File::open(filename);
    let file = match hfile {
        Ok(f) => f,
        Err(e) => panic!("Error opening file {:}", e),
    };
    
    let reader = BufReader::new(file);
    
    let mut signals: Vec<String> = Vec::new();
    for line in reader.lines(){
        match line {
            Ok(l) => signals.push(l),
            Err(_e) => panic!("Error reading file line"),
        }
    }
    signals
}

fn check_condition(map: &HashMap<char, i32>) -> bool {
    for (c, count) in map {
        if *count > 1 {
            return false;
        }
    }
    true
}


fn find_start(signal: &String, marker_length: usize) -> usize {
    // Find marker_length unique chars contig from the begining of the signal
    
    let mut head: usize = 0;
    let mut tail: usize = 0;
    let mut map: HashMap<char, i32> = HashMap::new();

    while head < signal.len() {
        let head_char = signal.chars().nth(head).unwrap();
        let tail_char = signal.chars().nth(tail).unwrap();

        if head < marker_length {
            match map.get(&head_char) {
                Some(count) => map.insert(head_char, count + 1),
                None => map.insert(head_char, 1),
            };
            head+=1;
            continue;
        }

        if check_condition(&map) {

            let head_marker = std::iter::repeat("|").take(head).collect::<String>();
            let tail_marker = std::iter::repeat("|").take(tail).collect::<String>();

            println!("");
            println!("{:}", head_marker);
            println!("{:}", signal);
            println!("{:}", tail_marker);
            println!("");

            return head
        }
        
        // add new vaue
        match map.get(&head_char) {
            Some(count) => map.insert(head_char, count+1),
            None => map.insert(head_char, 1),
        };
        
        //remove old
        match map.get(&tail_char) {
            Some(count) => map.insert(tail_char, count-1),
            None => panic!("Char not found"),
        };

        head+=1;
        tail+=1;
    }
    0
}

fn main() {
    let signals: Vec<String> = load_buffer("test_data.txt".to_string());
    for line in signals {
        let start: usize = find_start(&line, 14);
        println!("Signal: {0:<10} Message starting point: {1:<10} : Singal length: {2:<10}", &line, start, &line.len());
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_find_start_ok() {
        assert_eq!(find_start(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(find_start(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(find_start(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(find_start(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(find_start(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
}
