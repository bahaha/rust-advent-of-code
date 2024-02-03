use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeColor {
    Blue,
    Green,
    Red,
}

struct CubePackage {
    cubes: HashMap<CubeColor, u32>,
}

struct Cube {
    quantity: u32,
    color: CubeColor,
}

struct GameSet {
    cubes: Vec<Cube>,
}

struct Game {
    id: u32,
    game_sets: Vec<GameSet>,
}

impl CubeColor {
    fn from(s: &str) -> Option<Self> {
        match s {
            "blue" => Some(Self::Blue),
            "green" => Some(Self::Green),
            "red" => Some(Self::Red),
            _ => None,
        }
    }
}

impl CubePackage {
    fn new(total_blue_cubes: u32, total_green_cubes: u32, total_red_cubes: u32) -> Self {
        let mut cubes = HashMap::new();
        cubes.insert(CubeColor::Blue, total_blue_cubes);
        cubes.insert(CubeColor::Green, total_green_cubes);
        cubes.insert(CubeColor::Red, total_red_cubes);
        Self { cubes }
    }

    fn get(&self, color: &CubeColor) -> u32 {
        *self.cubes.get(&color).unwrap_or(&0)
    }
}

impl Cube {
    fn from(s: &str) -> Option<Self> {
        let mut iter = s.trim().split_whitespace();
        let quantity = iter.next()?.parse().ok()?;
        let color = CubeColor::from(iter.next()?)?;
        Some(Self { quantity, color })
    }
}

impl GameSet {
    fn from(s: &str) -> Option<Self> {
        let cubes = s.split(',').filter_map(Cube::from).collect();
        Some(Self { cubes })
    }

    fn is_valid(&self, package: &CubePackage) -> bool {
        self.cubes
            .iter()
            .all(|cube| cube.quantity <= package.get(&cube.color))
    }
}

impl Game {
    fn new(game_box: &str) -> Option<Self> {
        let mut iter = game_box.splitn(2, ": ");
        let id = iter.next()?.trim_start_matches("Game ").parse().ok()?;
        let game_sets = iter.next()?.split(';').filter_map(GameSet::from).collect();
        Some(Self { id, game_sets })
    }

    fn is_valid(&self, package: &CubePackage) -> bool {
        self.game_sets
            .iter()
            .all(|game_set| game_set.is_valid(package))
    }
}

fn play_game(game_box: &str, package: &CubePackage) -> u32 {
    game_box
        .lines()
        .filter_map(|line| Game::new(line))
        .filter(|game| game.is_valid(package))
        .map(|game| game.id)
        .sum()
}

fn main() {
    let game_box = include_str!("./test_data/sample.txt");
    let package = &CubePackage::new(14, 13, 12);
    let answer = play_game(game_box, package);
    println!("[Sample] Sum of total success ID: {}", answer);

    let game_box = include_str!("./test_data/input.txt");
    let package = &CubePackage::new(14, 13, 12);
    let answer = play_game(game_box, package);
    println!("Sum of total success ID: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_round_game_set() {
        let package = CubePackage::new(2, 2, 2);
        let game_set = GameSet::from("1 blue, 1 green; 2 green, 2 blue, 2 red");
        assert_eq!(game_set.unwrap().is_valid(&package), true);
        let game_set = GameSet::from("1 blue, 1 green; 2 green, 2 blue, 3 red");
        assert_eq!(game_set.unwrap().is_valid(&package), false);
    }

    #[test]
    fn test_game_sets() {
        let package = &CubePackage::new(14, 13, 12);
        let game_box = include_str!("./test_data/sample.txt");
        assert_eq!(play_game(game_box, &package), 8);

        let game_box = include_str!("./test_data/input.txt");
        assert_eq!(play_game(game_box, &package), 2061);
    }
}
