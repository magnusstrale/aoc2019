mod paint_robot;
mod intcode;

use intcode::{IntCode, IntCodeState};
use paint_robot::PaintRobot;

fn main() {
    let file_name = "src/day11.txt";
    let mut program = IntCode::file_to_program(&file_name);
    let mut robot = PaintRobot::new();
    paint(&mut program, &mut robot);
    println!("Colored {} tiles at least once", robot.count_colored_panels());

    println!("Re-running on white tile");
    let mut program = IntCode::file_to_program(&file_name);
    let mut robot = PaintRobot::new();
    robot.paint_here(1);
    paint(&mut program, &mut robot);
    robot.print_paint();
}

fn paint(program: &mut IntCode, robot: &mut PaintRobot) {
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