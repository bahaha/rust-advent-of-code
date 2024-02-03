use std::{cmp::max, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
}

impl Game {
    fn new(game_box: &str) -> Option<Self> {
        let mut iter = game_box.splitn(2, ": ");
        let _ = iter.next()?;
        let game_sets = iter.next()?.split(';').filter_map(GameSet::from).collect();
        Some(Self { game_sets })
    }

    fn get_cubes_package_threshold(&self) -> CubePackage {
        self.game_sets
            .iter()
            .fold(CubePackage::new(0, 0, 0), |mut acc, game_set| {
                for cube in &game_set.cubes {
                    let count = acc.cubes.entry(cube.color).or_insert(0);
                    *count = max(*count, cube.quantity);
                }
                acc
            })
    }

    fn get_cube_products_num(&self) -> u32 {
        let package = self.get_cubes_package_threshold();
        package.cubes.values().product()
    }
}

fn play_game(game_box: &str) -> u32 {
    game_box
        .lines()
        .filter_map(|line| Game::new(line))
        .map(|game| game.get_cube_products_num())
        .sum()
}

fn main() {
    let game_box = include_str!("./test_data/sample.txt");
    let answer = play_game(game_box);
    println!("[Sample] Sum of total cubes product num: {}", answer);

    let game_box = include_str!("./test_data/input.txt");
    let answer = play_game(game_box);
    println!("Sum of total cubes product num: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_round_game_set() {
        let game = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        let package = game.get_cubes_package_threshold();

        assert_eq!(*package.cubes.get(&CubeColor::Blue).unwrap_or(&0), 6);
        assert_eq!(*package.cubes.get(&CubeColor::Green).unwrap_or(&0), 2);
        assert_eq!(*package.cubes.get(&CubeColor::Red).unwrap_or(&0), 4);
        assert_eq!(game.get_cube_products_num(), 48);

        let game =
            Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
                .unwrap();
        let package = game.get_cubes_package_threshold();
        assert_eq!(*package.cubes.get(&CubeColor::Blue).unwrap_or(&0), 6);
        assert_eq!(*package.cubes.get(&CubeColor::Green).unwrap_or(&0), 13);
        assert_eq!(*package.cubes.get(&CubeColor::Red).unwrap_or(&0), 20);
        assert_eq!(game.get_cube_products_num(), 1560);
    }

    #[test]
    fn test_game_sets() {
        let game_box = include_str!("./test_data/sample.txt");
        assert_eq!(play_game(game_box), 2286);

        let game_box = include_str!("./test_data/input.txt");
        assert_eq!(play_game(game_box), 72596);
    }
}
