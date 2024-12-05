use std::{collections::{HashMap}, fs, str::Lines};


fn create_ordering_map(input: Lines) -> HashMap<i32, Vec<i32>> {
    let page_ordering_rules = input.take_while(|line| !line.is_empty())
                .map(|line| line.split_once("|").unwrap())
                .fold(HashMap::new(), |mut order: HashMap<i32, Vec<i32>>, item|{
                    let page_first: i32 = item.0.parse().unwrap();
                    let page_after: i32 = item.1.parse().unwrap();
                    if order.contains_key(&page_first) {
                        // update
                        if let Some(list) = order.get_mut(&page_first) {
                            list.push(page_after);
                        }
                    } else {
                        order.insert(page_first, Vec::from([page_after]));
                    }
                    order
                });
    page_ordering_rules
}


fn create_page_order(input: Lines) -> Vec<Vec<i32>> {
    let pages_per_update = input.skip_while(|line| !line.is_empty()).skip(1)
                        .map(|line| line.split(',').map(|i| i.parse().unwrap()).collect())
                        .collect();
    pages_per_update
}


fn is_correct_ordering(pages: &Vec<i32>, page_ordering_rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut must_be_before: Vec<i32> = Vec::new();

    for current_page in pages.iter().rev() {
        if must_be_before.contains(current_page) {
            return false;
        }
        page_ordering_rules.get(current_page).iter()
                           .flat_map(|&p| p)
                           .for_each(|&p| must_be_before.push(p));
    }

    true
}


fn count_correct_ordering(input: Lines, page_ordering_rules: HashMap<i32, Vec<i32>>) -> i32 {
    let mut total = 0;

    for pages in create_page_order(input) {
        let has_right_order = is_correct_ordering(&pages, &page_ordering_rules);
        if has_right_order {
            let middle_value : i32 = pages[pages.len() / 2];
            println!("RIGHT: {:?} => {}", &pages, middle_value);
            total += middle_value;
        } else {
            println!("WRONG: {:?}", &pages);
        }
    }
    total
}


fn count_incorrect_ordering(input: Lines, page_ordering_rules: HashMap<i32, Vec<i32>>) -> i32 {
    let empty_vec: Vec<i32> = Vec::new();
    let mut total = 0;

    for mut pages in create_page_order(input) {
        let has_right_order = is_correct_ordering(&pages, &page_ordering_rules);
        if !has_right_order {
            print!("REORDER: {:?} ", &pages);

            loop {
                let mut reordered = false;

                for cur_i in (0..pages.len()).rev() {
                    let priority = page_ordering_rules.get(&pages[cur_i]).unwrap_or(&empty_vec);
                    
                    for add_i in 0..cur_i {
                        if priority.contains(&pages[add_i]) {
                            pages.swap(cur_i, add_i);
                            reordered = true;
                            break;
                        }
                    }
                }

                if !reordered {
                    break;
                }
            }

            let middle_value : i32 = pages[pages.len() / 2];
            println!(" --> {:?} => {}", &pages, middle_value);
            total += middle_value;
        }
    }
    total
}


#[cfg(test)]
pub mod day5_tests {
    use super::*;

#[test]
fn example1() {
    let input = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let page_ordering_rules = create_ordering_map(input.lines());
    let total = count_correct_ordering(input.lines(), page_ordering_rules);
    assert_eq!(61 + 53 + 29, total); // 75,47,61,53,29  |  97,61,53,29,13  |  75,29,13
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day5/input.txt").unwrap();
    let page_ordering_rules = create_ordering_map(input.lines());
    let total = count_correct_ordering(input.lines(), page_ordering_rules);
    assert_eq!(5651, total);
}


#[test]
fn example2() {
    let input = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let page_ordering_rules = create_ordering_map(input.lines());
    let total = count_incorrect_ordering(input.lines(), page_ordering_rules);
    assert_eq!(47 + 29 + 47, total); // 75,97,47,61,53 => 97,75,47,61,53 | 61,13,29 => 61,29,13 | 97,13,75,29,47 => 97,75,47,29,13
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day5/input.txt").unwrap();
    let page_ordering_rules = create_ordering_map(input.lines());
    let total = count_incorrect_ordering(input.lines(), page_ordering_rules);
    assert_eq!(4743, total);
}

}