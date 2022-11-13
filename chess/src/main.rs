mod controller;
pub use controller::ChessController;

fn main() {
  let controller = ChessController::new();
  controller.start_game();
}
