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

#[derive(Debug)]
struct GamePlayData {
  elf_action: GameAction,
  player_action: GameAction
}

fn main() {
  let file_contents = fs::read_to_string("./res/input.txt");
  match file_contents {
    Ok(_file_content) => {
      println!("{:?}", calculate_score_for_round([
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
      ]));
    }
    Err(_) => {
      panic!("cannot read file!");
    }
  }
}

fn calculate_score_for_round(data: [GamePlayData; 3]) -> u32 {
  let player_selected_score = data
    .map(|x| get_selection_sore(x.player_action))
    .iter()
    .sum::<u32>();

  let game_play_score = data
    .map(|x| get_game_play_score(x))
    .iter()
    .sum::<u32>();

  return player_selected_score + game_play_score;
}

fn get_game_play_result(data: GamePlayData) -> GamePlayResult {
  if data.player_action == GameAction::Rock {
    if data.elf_action == GameAction::Rock {
      return GamePlayResult::Draw;
    }
    if data.elf_action == GameAction::Paper {
      return GamePlayResult::Lost;
    }
    if data.elf_action == GameAction::Scissors {
      return GamePlayResult::Won;
    }
  }

  if data.player_action == GameAction::Paper {
    if data.elf_action == GameAction::Rock {
      return GamePlayResult::Won;
    }
    if data.elf_action == GameAction::Paper {
      return GamePlayResult::Draw;
    }
    if data.elf_action == GameAction::Scissors {
      return GamePlayResult::Lost;
    }
  }
  if data.player_action == GameAction::Scissors {
    if data.elf_action == GameAction::Rock {
      return GamePlayResult::Lost;
    }
    if data.elf_action == GameAction::Paper {
      return GamePlayResult::Won;
    }
    if data.elf_action == GameAction::Scissors {
      return GamePlayResult::Draw;
    }
  }
  return GamePlayResult::Draw;
}

fn get_selection_sore(selection_option: GameAction) -> u32 {
  if selection_option == GameAction::Rock {
    return 1;
  }
  if selection_option == GameAction::Paper {
    return 2;
  }
  return 3;
}

fn get_game_play_score(data: GamePlayData) -> u32 {
  let result = get_game_play_result(data);
  if result == GamePlayResult::Draw {
    return 3;
  }
  if result == GamePlayResult::Won {
    return 6;
  }
  return 0;
}
