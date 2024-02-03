#[derive(Debug)]
struct Cube<'a> {
    n_cubes: u32,
    color: &'a str,
}

#[derive(Default, Debug)]
struct CubePackage {
    total_blue_cubes: u32,
    total_green_cubes: u32,
    total_red_cubes: u32,
}

#[derive(Debug)]
struct Game<'a> {
    package: &'a CubePackage,
}

impl<'a> Game<'a> {
    fn new(package: &'a CubePackage) -> Self {
        Game { package }
    }

    fn play(&mut self, game_box: &str) -> bool {
        let game_sets = game_box
            .split(";")
            .map(|game_sets| game_sets.trim().split(","))
            .map(|game_set| {
                game_set
                    .map(|cube_str| cube_str.trim().split_whitespace())
                    .filter_map(|mut game_set| match (game_set.next(), game_set.next()) {
                        (Some(n_cubes), Some(color)) if n_cubes.parse::<u32>().is_ok() => {
                            Some(Cube {
                                n_cubes: n_cubes.parse().unwrap(),
                                color,
                            })
                        }
                        _ => None,
                    })
                    .collect::<Vec<Cube>>()
            })
            .collect::<Vec<Vec<Cube>>>();

        for game_set in game_sets {
            for cube in game_set {
                let bad_game = match cube.color {
                    "blue" => cube.n_cubes > self.package.total_blue_cubes,
                    "green" => cube.n_cubes > self.package.total_green_cubes,
                    "red" => cube.n_cubes > self.package.total_red_cubes,
                    _ => true,
                };

                if bad_game {
                    return false;
                }
            }
        }

        true
    }
}

fn play_game(game_box: &str, package: &CubePackage) -> u32 {
    let game_prefix_len = "Game ".len();
    let mut game = Game::new(package);
    let good_game_id = game_box.lines().flat_map(|line| {
        line.split_once(": ")
            .filter(|&(_, game_box)| game.play(game_box))
            .and_then(|(game_label, _)| {
                let game_id = &game_label[game_prefix_len..];
                game_id.parse::<u32>().ok()
            })
    });

    good_game_id.sum::<u32>()
}

fn main() {
    let game_box = include_str!("./test_data/sample.txt");
    let package = &CubePackage {
        total_blue_cubes: 14,
        total_green_cubes: 13,
        total_red_cubes: 12,
    };
    let answer = play_game(game_box, package);
    println!("[Sample] Sum of total success ID: {}", answer);

    let game_box = include_str!("./test_data/input.txt");
    let package = &CubePackage {
        total_blue_cubes: 14,
        total_green_cubes: 13,
        total_red_cubes: 12,
    };
    let answer = play_game(game_box, package);
    println!("Sum of total success ID: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_round_game_set() {
        let package = CubePackage {
            total_blue_cubes: 2,
            total_green_cubes: 2,
            total_red_cubes: 2,
        };
        let mut game = Game::new(&package);
        assert_eq!(game.play("1 blue, 1 green; 2 green, 2 blue, 2 red"), true);
        assert_eq!(game.play("3 red"), false);
    }

    #[test]
    fn test_game_sets() {
        let package = CubePackage {
            total_blue_cubes: 14,
            total_green_cubes: 13,
            total_red_cubes: 12,
        };
        let game_box = include_str!("./test_data/sample.txt");
        assert_eq!(play_game(game_box, &package), 8);

        let game_box = include_str!("./test_data/input.txt");
        assert_eq!(play_game(game_box, &package), 2061);
    }
}
