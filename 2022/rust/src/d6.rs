use std::fs;

use crate::util::log_result;

use self::communicator::Communicator;

mod communicator;
mod string_has_unique_chars;

pub fn run() {
    let signals = fs::read_to_string("data/d6.txt").expect("could not read input file");
    let communicator = Communicator { signals };

    log_result(communicator.get_start_of_packet_index(), "d6p1");
    log_result(communicator.get_start_of_message_index(), "d6p2");
}
