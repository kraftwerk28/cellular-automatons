use std::io;

mod ant;
mod game_of_life;

fn main() {
  let prompt = "Choose what to play:\n\
                1) Game of life\n\
                2) Langton's Ant";
  println!("{}", prompt);
  let mut choice: String = "".to_string();
  io::stdin().read_line(&mut choice).unwrap();

  match choice[..1].parse::<u8>().expect("Failed to parse string") {
    1 => {
      game_of_life::init();
    }
    2 => {
      ant::init();
    }
    _ => println!("Select proper choice"),
  }
}
