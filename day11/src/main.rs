mod paint_robot;
mod intcode;

use intcode::{IntCode, IntCodeState};
use paint_robot::PaintRobot;

const FILE_NAME: &str = "src/day11.txt";

fn main() {
    let mut robot = PaintRobot::new();
    paint(&mut robot);
    println!("Colored {} tiles at least once", robot.count_colored_panels());

    println!("Re-running on white tile");
    let mut robot = PaintRobot::new();
    robot.paint_here(1);
    paint(&mut robot);
    robot.print_panels();
}

fn paint(robot: &mut PaintRobot) {
    let mut program = IntCode::file_to_program(&FILE_NAME);
    let mut paint = true;
    loop {
        match &program.run_slice() {
            IntCodeState::NeedInput => program.add_input(robot.get_color_here()),
            IntCodeState::Output(data) => {
                if paint { 
                    robot.paint_here(*data);
                } else {
                    robot.turn_and_move(*data);
                }
                paint = !paint;
            },
            IntCodeState::Done => return
        }
    }
}