use std::fs;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
struct Cubes {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

#[derive(Debug)]
struct Game {
    pub id: i32,
    pub revealed_subsets: Vec<Cubes>,
}

fn colored_cubes_count(color: &str, input: &str) -> i32 {
    if input.find(color).is_some() {
        return input.split_whitespace().next().unwrap().parse::<i32>().unwrap();
    }
    return 0;
}



impl From<&str> for Cubes {

    fn from(input: &str) -> Self {
        input.split(",")
              .map(|colored_dice| (
                colored_cubes_count("red", colored_dice),
                colored_cubes_count("green", colored_dice),
                colored_cubes_count("blue", colored_dice),
              ))
              .fold(Cubes { red: 0, green: 0, blue: 0,},
                |accum, item| Cubes {
                                                            red: accum.red + item.0, 
                                                            green: accum.green + item.1, 
                                                            blue: accum.blue + item.2,
                                                        }
              )
    }
}

impl Cubes {

    fn has_enough_for(&self, other: &Cubes) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (game_id, games) = line[5..].split_once(":").unwrap();
        let subsets = games.split(";")
                                        .map(Cubes::from)
                                        .collect::<Vec<Cubes>>();

        Game {
            id: game_id.parse::<i32>().unwrap(),
            revealed_subsets: subsets,
        }
    }
}


impl Game {

    fn is_not_possible(&self, with_cube: &Cubes) -> bool {
        self.revealed_subsets.iter()
                             .any(|cubes| !cubes.has_enough_for(with_cube))
    }

    fn smallest_possible_cubes(&self) -> Cubes {
        Cubes {
            red: self.revealed_subsets.iter().map(|c| c.red).max().unwrap(),
            green: self.revealed_subsets.iter().map(|c| c.green).max().unwrap(),
            blue: self.revealed_subsets.iter().map(|c| c.blue).max().unwrap(),
        }
    }

}


#[test]
fn example1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"; // games

    let bag_content = Cubes {red: 12, green: 13, blue: 14,};
    let mut possible_games= 0;

    for game in input.lines().map(Game::from).collect::<Vec<Game>>() {
        println!("game {} is possible {}", game.id, game.is_not_possible(&bag_content));
        if !game.is_not_possible(&bag_content) {
            possible_games += game.id;
        }
    }

    assert_eq!(8, possible_games);
}

#[test]
fn part1() {
    let bag_content = Cubes {red: 12, green: 13, blue: 14,};
    let mut possible_games= 0;

    let input = fs::read_to_string("src/input.txt").unwrap();
    for game in input.lines().map(Game::from).collect::<Vec<Game>>() {
        println!("game {} is possible {}", game.id, game.is_not_possible(&bag_content));
        if !game.is_not_possible(&bag_content) {
            possible_games += game.id;
        }
    }

    println!("sum: {}", possible_games);
}




#[test]
fn example2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"; // games

    let bag_content = Cubes {red: 12, green: 13, blue: 14,};
    let mut possible_games= 0;
    let games = input.lines().map(Game::from).collect::<Vec<Game>>();

    for game in &games {
        let smalles_cubes = game.smallest_possible_cubes();
        println!("game {} is possible with {:?} => {}", game.id, &smalles_cubes, smalles_cubes.power());
    }

    assert_eq!(1, games.get(0).unwrap().id);
    assert_eq!(Cubes{ red: 4, green: 2, blue: 6}, games.get(0).unwrap().smallest_possible_cubes());
    assert_eq!(48, games.get(0).unwrap().smallest_possible_cubes().power());
    assert_eq!(2, games.get(1).unwrap().id);
    assert_eq!(Cubes{ red: 1, green: 3, blue: 4}, games.get(1).unwrap().smallest_possible_cubes());
    assert_eq!(12, games.get(1).unwrap().smallest_possible_cubes().power());
    assert_eq!(3, games.get(2).unwrap().id);
    assert_eq!(Cubes{ red: 20, green: 13, blue: 6}, games.get(2).unwrap().smallest_possible_cubes());
    assert_eq!(1560, games.get(2).unwrap().smallest_possible_cubes().power());
    assert_eq!(4, games.get(3).unwrap().id);
    assert_eq!(Cubes{ red: 14, green: 3, blue: 15}, games.get(3).unwrap().smallest_possible_cubes());
    assert_eq!(630, games.get(3).unwrap().smallest_possible_cubes().power());
    assert_eq!(5, games.get(4).unwrap().id);
    assert_eq!(Cubes{ red: 6, green: 3, blue: 2}, games.get(4).unwrap().smallest_possible_cubes());
    assert_eq!(36, games.get(4).unwrap().smallest_possible_cubes().power());
}



#[test]
fn part2() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let power_of_sets: i32 = input.lines()
                                        .map(Game::from)
                                        .map(|game| game.smallest_possible_cubes())
                                        .map(|gsc| gsc.power())
                                        .sum();

    println!("sum of the power: {}", power_of_sets);
}

