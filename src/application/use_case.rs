use crate::application::compute_available_moves_use_case::ComputeAvailableMovesUseCase;
use crate::application::evaluate_game_end_use_case::EvaluateGameEndUseCase;
use crate::application::play_ai_move_use_case::PlayAIMoveUseCase;
use crate::application::play_move_use_case::PlayMoveUseCase;
use crate::application::start_game_use_case::StartGameUseCase;

pub struct UseCase {
    pub initialize_game_use_case: StartGameUseCase,
    pub compute_available_moves_use_case: ComputeAvailableMovesUseCase,
    pub play_move_use_case: PlayMoveUseCase,
    pub play_ai_move_use_case: PlayAIMoveUseCase,
    pub evaluate_game_end_use_case: EvaluateGameEndUseCase,
}

impl UseCase {
    pub fn new() -> Self {
        Self {
            initialize_game_use_case: StartGameUseCase {},
            compute_available_moves_use_case: ComputeAvailableMovesUseCase {},
            play_move_use_case: PlayMoveUseCase {},
            play_ai_move_use_case: PlayAIMoveUseCase {},
            evaluate_game_end_use_case: EvaluateGameEndUseCase {},
        }
    }
}
