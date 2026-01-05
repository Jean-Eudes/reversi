use crate::application::ai_move_use_case::AIMoveUseCase;
use crate::application::compute_available_moves_use_case::ComputeAvailableMovesUseCase;
use crate::application::evaluate_game_end_use_case::EvaluateGameEndUseCase;
use crate::application::move_use_case::MoveUseCaseImpl;
use crate::application::player_move_use_case::PlayerMoveUseCase;
use crate::application::start_game_use_case::StartGameUseCase;

pub struct UseCase {
    pub initialize_game_use_case: StartGameUseCase,
    pub compute_available_moves_use_case: ComputeAvailableMovesUseCase,
    pub play_move_use_case: PlayerMoveUseCase,
    pub play_ai_move_use_case: AIMoveUseCase,
    pub evaluate_game_end_use_case: EvaluateGameEndUseCase,
}

impl UseCase {
    pub fn new() -> Self {
        Self {
            initialize_game_use_case: StartGameUseCase {},
            compute_available_moves_use_case: ComputeAvailableMovesUseCase {},
            play_move_use_case: PlayerMoveUseCase::new(Box::new(MoveUseCaseImpl {})),
            play_ai_move_use_case: AIMoveUseCase::new(Box::new(MoveUseCaseImpl {})),
            evaluate_game_end_use_case: EvaluateGameEndUseCase {},
        }
    }
}
