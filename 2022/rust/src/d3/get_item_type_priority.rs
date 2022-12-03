const LOWERCASE_INDEX_OFFSET: u32 = 96;
const UPPERCASE_INDEX_OFFSET: u32 = 64;

/**
 * Returns the numbers 1-26 for a-z and 27-52 for A-Z
 */
pub fn get_item_type_priority(item_type: char) -> u32 {
    if item_type.is_lowercase() {
        // Subtract ASCII index offset to get numbers 1-26 for a-z
        return item_type as u32 - LOWERCASE_INDEX_OFFSET;
    } else {
        // Subtract ASCII index offset minus previous 1-26 indexes of a-z to get the numbers 27-52 for A-Z
        return item_type as u32 - (UPPERCASE_INDEX_OFFSET - 26);
    }
}
