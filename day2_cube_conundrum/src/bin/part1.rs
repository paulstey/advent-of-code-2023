use day2_cube_conundrum::{lines_from_file, Game};
use time::Instant;

fn main() {
    let t1 = Instant::now();

    let lines = lines_from_file("data/day2.txt");

    let games = lines.iter().map(|input| Game::from(&input));

    let mut score = 0;

    for (i, game) in games.enumerate() {
        score += if game.is_possible() { i + 1 } else { 0 }
    }
    println!("{:?}", Instant::now() - t1);
    println!("{:?}", score);
}
