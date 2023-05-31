use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum GamePlayResult {
  Won,
  Lost,
  Draw
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameAction {
  Rock,
  Paper,
  Scissors
}

#[derive(Debug, Copy, Clone)]
struct GamePlayData {
  elf_action: GameAction,
  player_action: GameAction
}

fn main() {
  let file_contents = fs::read_to_string("./res/input.txt");
  match file_contents {
    Ok(_file_content) => {
      let data = [
        GamePlayData {
          player_action: GameAction::Rock,
          elf_action: GameAction::Paper
        },
        GamePlayData {
          player_action: GameAction::Paper,
          elf_action: GameAction::Rock
        },
        GamePlayData {
          player_action: GameAction::Scissors,
          elf_action: GameAction::Scissors
        }
      ];
      println!("{:?}", calculate_score_for_round(data));
    }
    Err(_) => {
      panic!("cannot read file!");
    }
  }
}

fn calculate_score_for_round(data: [GamePlayData; 3]) -> u32 {
  let player_selected_score = data
    .map(|x| get_player_action_score(&x.player_action))
    .iter()
    .sum::<u32>();

  let game_play_score = data
    .map(|x| get_game_play_score(&x))
    .iter()
    .sum::<u32>();

  return player_selected_score + game_play_score;
}

fn get_game_play_result(player_action: GameAction, elf_action: GameAction) -> GamePlayResult {
  match (player_action, elf_action) {
    (GameAction::Rock, GameAction::Rock) => GamePlayResult::Draw,
    (GameAction::Rock, GameAction::Paper) => GamePlayResult::Lost,
    (GameAction::Rock, GameAction::Scissors) => GamePlayResult::Won,
    (GameAction::Paper, GameAction::Rock) => GamePlayResult::Won,
    (GameAction::Paper, GameAction::Paper) => GamePlayResult::Draw,
    (GameAction::Paper, GameAction::Scissors) => GamePlayResult::Lost,
    (GameAction::Scissors, GameAction::Rock) => GamePlayResult::Lost,
    (GameAction::Scissors, GameAction::Paper) => GamePlayResult::Won,
    (GameAction::Scissors, GameAction::Scissors) => GamePlayResult::Draw
  }
}

fn get_player_action_score(action: &GameAction) -> u32 {
  if action == &GameAction::Rock {
    return 1;
  }
  if action == &GameAction::Paper {
    return 2;
  }
  return 3;
}

fn get_game_play_score(data: &GamePlayData) -> u32 {
  let result = get_game_play_result(data.player_action, data.elf_action);
  if result == GamePlayResult::Draw {
    return 3;
  }
  if result == GamePlayResult::Won {
    return 6;
  }
  return 0;
}
