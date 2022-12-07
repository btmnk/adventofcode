pub enum ParsedLine {
    MoveInto,
    MoveOut,
    MoveToRoot,
    List,
    File,
    Directory,
}

pub fn parse_line(input: &str) -> ParsedLine {
    if input == "$ cd /" {
        return ParsedLine::MoveToRoot;
    }

    if input == "$ cd .." {
        return ParsedLine::MoveOut;
    }

    if input.starts_with("$ cd ") {
        return ParsedLine::MoveInto;
    }

    if input.starts_with("$ ls") {
        return ParsedLine::List;
    }

    if input.starts_with("dir ") {
        return ParsedLine::Directory;
    }

    return ParsedLine::File;
}
