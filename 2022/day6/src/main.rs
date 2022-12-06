use std::convert::TryInto;

fn detect_marker(input: &str, size: u32) -> u32 {
    let ns = size as usize;
    for index in 0..(input.len() - ns) {
        if !has_duplicate(&input[index..index+ns]) {
          return (index + ns).try_into().unwrap(); 
        }
    }
    return u32::MIN;
}

fn has_duplicate(part: &str) -> bool {
    for (i,left) in part.char_indices() {
        match part[i+1..].find(left) {
            None => {},
            Some(_) => return true,
        };
    }
    return false;
}


fn main() {
    let input = [
        "bvwbjplbgvbhsrlpgdmjqwftvncz", //: first marker after character 5
        "nppdvjthqldpwncqszvftbrmjlhg", //: first marker after character 6
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", //: first marker after character 10
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", //: first marker after character 11
        
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb", // first marker after character 19
        "bvwbjplbgvbhsrlpgdmjqwftvncz", // first marker after character 23
        "nppdvjthqldpwncqszvftbrmjlhg", // first marker after character 23
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", // first marker after character 29
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", // first marker after character 26
    ];

    for i in &input[..4] {
        println!("text: {}, marker: {}", &i, detect_marker(i, 4));
    }
    for i in &input[4..] {
        println!("text: {}, marker: {}", &i, detect_marker(i, 14));
    }
}
