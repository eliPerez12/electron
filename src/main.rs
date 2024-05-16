

enum Operation {
    ADD
}

impl Operation {
    fn from_str(string: String) -> Result<Self, ()> {
        match string.to_uppercase().as_str() {
            "ADD" => Ok(Operation::ADD),
            _ => Err(())
        }
    }
}


struct InstructionLine {
    operation: Operation,
    a: u8,
    b: u8,
}


fn main() {
    let mut file = std::fs::File::open("program.elt").unwrap();
    let mut buffer = String::new();
    std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();

    for line in buffer.lines() {
        let mut line = line.to_string();
        let comment = line.find(';');
        if let Some(comment) = comment{
            line.truncate(comment);
        }

        let words: Vec<String> = line.split_whitespace().map(|s|s.to_owned()).collect();
        println!("{:?}", words);
    }

}
