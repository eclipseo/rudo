pub struct Comm<'a> {
    value: Vec<&'a str>,
    pub program: String,
    pub args: Vec<&'a str>,
}

impl<'a> Comm<'a> {
    pub fn new(mut value: Vec<&'a str>) -> Self {
        let mut program = String::new();
        program.push_str(value[0]);
        value.remove(0);
        let args = value.clone();
        Self {
            value,
            program,
            args,
        }
    }
}
