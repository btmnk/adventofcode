use super::string_has_unique_chars::string_has_unique_chars;

pub struct Communicator {
    pub signals: String,
}

impl Communicator {
    pub fn get_start_of_packet_index(&self) -> Option<usize> {
        return self.get_marker_end_index(4);
    }

    pub fn get_start_of_message_index(&self) -> Option<usize> {
        return self.get_marker_end_index(14);
    }

    fn get_marker_end_index(&self, signal_size: usize) -> Option<usize> {
        let last_possible_index = self.signals.len() - (signal_size - 1);

        let mut target_index: Option<usize> = None;

        for signal_start in 0..last_possible_index {
            let signal_end = signal_start + signal_size;
            let signal = &self.signals[signal_start..signal_end];

            if string_has_unique_chars(signal) {
                target_index = Some(signal_end);
                break;
            }
        }

        return target_index;
    }
}
