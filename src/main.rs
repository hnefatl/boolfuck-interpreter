// https://esolangs.org/wiki/Boolfuck
// https://www.codewars.com/kata/5861487fdb20cff3ab000030

extern crate boolfuck;
use boolfuck::*;

fn main() {
    let code = "";
    let input = vec![];
    let mut state = State::new(code.chars().collect(), input);
    while state.step() {}
}
