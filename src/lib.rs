use std::collections::HashSet;
enum Command {
    Draai(i32),
    Loop(i32),
    Spring(i32),
}

fn parse_command(input: &str) -> Command {
    if let Some(amount) = input.strip_prefix("draai ") {
        return Command::Draai(amount.parse().unwrap());
    }
    else if let Some(amount) = input.strip_prefix("loop "){
        return Command::Loop(amount.parse().unwrap());
    }
    else if let Some(amount) = input.strip_prefix("spring "){
        return Command::Spring(amount.parse().unwrap());
    }
    panic!()
}

struct State {
    position: (i32, i32),
    direction: i32,
}

fn get_direction_vec(direction: i32) -> (i32, i32) {
    match direction {
        0 => {(0, 1)},
        45 => {(1, 1)},
        90 => {(1, 0)},
        135 => {(1, -1)},
        180 => {(0, -1)},
        225 => {(-1, -1)},
        270 => {(-1, 0)},
        315 => {(-1, 1)},
        _ => panic!(),
    }
}
fn exec_command(command: Command, state: &mut State) {
    match command {
        Command::Draai(amount) => { state.direction += amount;
                                    state.direction = state.direction % 360;
                                    if state.direction < 0 {
                                        state.direction += 360;
                                    }
        },
        Command::Loop(amount) => {
            let direction_vec = get_direction_vec(state.direction);
            state.position.0 += direction_vec.0 * amount;            
            state.position.1 += direction_vec.1 * amount;            
        },
        Command::Spring(amount) => {
            let direction_vec = get_direction_vec(state.direction);
            state.position.0 += direction_vec.0 * amount;            
            state.position.1 += direction_vec.1 * amount;            
        },
    }
}

pub fn do_part_1(input: &str) -> usize {
    let mut state = State { position: (0, 0), direction: 0};
    for line in input.lines() {
        let command = parse_command(line);
        exec_command(command, &mut state);
    }

    (state.position.0.abs() + state.position.1.abs()) as _
}


fn exec_command_paint(command: Command, state: &mut State, painted_points: &mut HashSet<(i32, i32)>) {
    match command {
        Command::Draai(amount) => { state.direction += amount;
                                    state.direction = state.direction % 360;
                                    if state.direction < 0 {
                                        state.direction += 360;
                                    }
        },
        Command::Loop(amount) => {
            let direction_vec = get_direction_vec(state.direction);
            
            let sign = amount / amount.abs();
            for i in 0..=amount.abs() {
                painted_points.insert((state.position.0 + sign * i * direction_vec.0, state.position.1 + sign * i * direction_vec.1)); 
            }

            state.position.0 += direction_vec.0 * amount;            
            state.position.1 += direction_vec.1 * amount;            
        },
        Command::Spring(amount) => {
            let direction_vec = get_direction_vec(state.direction);
            state.position.0 += direction_vec.0 * amount;            
            state.position.1 += direction_vec.1 * amount;            
            painted_points.insert((state.position.0, state.position.1)); 
        },
    }
}

pub fn do_part_2(input: &str) -> HashSet<(i32, i32)>{
    let mut state = State { position: (0, 0), direction: 0};
    let mut painted_points = HashSet::new();
    for line in input.lines() {
        let command = parse_command(line);
        exec_command_paint(command, &mut state, &mut painted_points);
    }
    painted_points
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "draai 90
loop 6
spring 2
draai -45
loop 2";

    #[test]
    fn test_part_1() {
        let distance = do_part_1(INPUT);
        assert_eq!(distance, 12);
    }
}
