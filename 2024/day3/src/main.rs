use std::{collections::HashMap, fs};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum ValidState {
    LiteralM,
    LiteralU,
    LiteralL,
    LiteralLparen,
    LiteralRparen,
    Num,
    LiteralComma,
    LiteralD,
    LiteralO,
    LiteralN,
    LiteralT,
    LiteralApos,
}

fn parse_stack(stack: &Vec<(ValidState, i32)>, flag: i32) -> i32 {
    let mut op_a = 0;
    let mut op_b = 0;
    let mut pre_comma = true;
    for (state, value) in stack {
        if *state == ValidState::Num && pre_comma {
            op_a = op_a * 10 + *value;
        }
        if *state == ValidState::Num && !pre_comma {
            op_b = op_b * 10 + *value;
        }
        if *state == ValidState::LiteralComma {
            pre_comma = false;
        }
    }
    op_a * op_b * flag
}

struct StateMachine {
    sum: i32,
    flag: i32,
    stack: Vec<(ValidState, i32)>,
    do_stack: Vec<ValidState>,
    transition_table: HashMap<ValidState, Vec<ValidState>>,
    do_transition_table: HashMap<ValidState, ValidState>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            sum: 0,
            flag: 1,
            do_stack: Vec::new(),
            stack: Vec::new(),
            transition_table: HashMap::new(),
            do_transition_table: HashMap::new(),
        }
    }

    fn init_transitions(&mut self) {
        self.transition_table
            .insert(ValidState::LiteralM, [ValidState::LiteralU].to_vec());
        self.transition_table
            .insert(ValidState::LiteralU, [ValidState::LiteralL].to_vec());
        self.transition_table
            .insert(ValidState::LiteralL, [ValidState::LiteralLparen].to_vec());
        self.transition_table
            .insert(ValidState::LiteralLparen, [ValidState::Num].to_vec());
        self.transition_table.insert(
            ValidState::Num,
            [
                ValidState::LiteralRparen,
                ValidState::Num,
                ValidState::LiteralComma,
            ]
            .to_vec(),
        );
        self.transition_table
            .insert(ValidState::LiteralComma, [ValidState::Num].to_vec());
    }

    fn extend_transitions(&mut self) {
        self.do_transition_table
            .insert(ValidState::LiteralD, ValidState::LiteralO);
        self.do_transition_table
            .insert(ValidState::LiteralO, ValidState::LiteralN);
        self.do_transition_table
            .insert(ValidState::LiteralN, ValidState::LiteralApos);
        self.do_transition_table
            .insert(ValidState::LiteralApos, ValidState::LiteralT);
    }

    fn make_transitions(&mut self, state: ValidState, val: i32) {
        match self.stack.last() {
            Some(current_state) => match self.transition_table.get(&current_state.0) {
                Some(next_state) => {
                    if let Some(_a) = (*next_state).iter().find(|x| **x == state) {
                        self.stack.push((state, val));
                    } else {
                        self.stack.clear();
                    }
                    if state == ValidState::LiteralRparen {
                        let mult = parse_stack(&self.stack, self.flag);
                        self.sum += mult;
                        self.stack.clear();
                    }
                }

                None => {
                    self.stack.clear();
                }
            },
            None => {
                if state == ValidState::LiteralM {
                    self.stack.push((state, val));
                }
            }
        }
    }

    fn make_do_transitions(&mut self, state: ValidState) {
        match self.do_stack.last() {
            Some(last_state) => match self.do_transition_table.get(last_state) {
                Some(next_state) => {
                    if *next_state == state {
                        self.do_stack.push(state);
                    }
                    if state == ValidState::LiteralO {
                        self.flag = 1;
                    }
                    if state == ValidState::LiteralT {
                        self.flag = 0;
                        self.do_stack.clear();
                    }
                }
                None => {}
            },
            None => {
                self.do_stack.push(state);
            }
        }
    }
}

fn part1() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut state_machine = StateMachine::new();
    state_machine.init_transitions();
    for line in buffer.lines() {
        for char in line.chars() {
            match char {
                '(' => {
                    state_machine.make_transitions(ValidState::LiteralLparen, 0);
                }
                ')' => {
                    state_machine.make_transitions(ValidState::LiteralRparen, 0);
                }
                ',' => {
                    state_machine.make_transitions(ValidState::LiteralComma, 0);
                }
                'm' => {
                    state_machine.make_transitions(ValidState::LiteralM, 0);
                }
                'u' => {
                    state_machine.make_transitions(ValidState::LiteralU, 0);
                }
                'l' => {
                    state_machine.make_transitions(ValidState::LiteralL, 0);
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    state_machine.make_transitions(
                        ValidState::Num,
                        i32::try_from(char.to_digit(10).unwrap()).unwrap(),
                    );
                }
                _ => state_machine.stack.clear(),
            }
        }
    }

    println!("Part 1: {}", state_machine.sum);
}

fn part2() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut state_machine = StateMachine::new();
    state_machine.init_transitions();
    state_machine.extend_transitions();
    for line in buffer.lines() {
        for char in line.chars() {
            match char {
                'd' => {
                    state_machine.make_do_transitions(ValidState::LiteralD);
                }
                'o' => {
                    state_machine.make_do_transitions(ValidState::LiteralO);
                }
                'n' => {
                    state_machine.make_do_transitions(ValidState::LiteralN);
                }
                '\'' => {
                    state_machine.make_do_transitions(ValidState::LiteralApos);
                }
                't' => {
                    state_machine.make_do_transitions(ValidState::LiteralT);
                }
                '(' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(ValidState::LiteralLparen, 0);
                }
                ')' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(ValidState::LiteralRparen, 0);
                }
                ',' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(ValidState::LiteralComma, 0);
                }
                'm' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(ValidState::LiteralM, 0);
                }
                'u' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(ValidState::LiteralU, 0);
                }
                'l' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(ValidState::LiteralL, 0);
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    state_machine.do_stack.clear();
                    state_machine.make_transitions(
                        ValidState::Num,
                        i32::try_from(char.to_digit(10).unwrap()).unwrap(),
                    );
                }
                _ => {
                    state_machine.stack.clear();
                    state_machine.do_stack.clear();
                }
            }
        }
    }

    println!("Part 2: {}", state_machine.sum);
}

fn main() {
    part1();
    part2()
}
