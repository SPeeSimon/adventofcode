
#[derive(Debug)]
struct Elf {
    carried: u32,
}


impl Elf {
    fn add(&mut self, line: &str) {
        self.carried += line.parse::<u32>().unwrap();
    }
}

fn main() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    
    let mut elves: Vec<Elf> = Vec::new();
    elves.push(Elf{
        carried: 0,
    });

    for line in input.lines() {
        if line.is_empty() {
            elves.push(Elf{
                carried: 0,
            });
        } else {
            elves.last_mut().unwrap().add(line)
        }
    }

    elves.sort_by_key(|x: &Elf| x.carried);
    println!("total elves: {}", elves.len());
    println!("carried most: {}", elves.last().unwrap().carried);
}
