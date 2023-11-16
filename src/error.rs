use colored::Colorize;

#[derive(Debug)]
pub struct CodeError {
    line: usize,
    start: usize,
    end: usize,
    msg: String,
}
impl CodeError {
    pub fn new(line: usize, start: usize, end: usize, msg: &str) -> Self {
        CodeError {
            line,
            start,
            end,
            msg: msg.to_owned(),
        }
    }
    pub fn print_error(&self, input: &String) {
        let lines: Vec<&str> = input.split("\n").collect();
        if let Some(line) = lines.get(self.line - 1) {
            let mut start_col = self.start
                - lines
                    .iter()
                    .take(self.line - 1)
                    .map(|s| s.len() + 1)
                    .sum::<usize>();
            let end_col = start_col + (self.end - self.start);
            start_col = if start_col == 0 {
                start_col
            } else {
                start_col - 1
            };

            println!("{} | ", " ".repeat(self.line.to_string().len()));
            print!("{} | ", self.line.to_string().yellow());
            print!(
                "{}",
                line.chars().skip(0).take(start_col).collect::<String>()
            );
            print!(
                "{}",
                line.chars()
                    .skip(start_col)
                    .take(end_col - start_col)
                    .collect::<String>()
                    .red()
                    .bold()
            );
            println!(
                "{}",
                line.chars()
                    .skip(end_col)
                    .take(line.len() + 1)
                    .collect::<String>()
            );
            print!("{} | ", " ".repeat(self.line.to_string().len()));
            print!("{}", " ".repeat(start_col));
            for i in start_col..end_col {
                print!(
                    "{}",
                    (if i == start_col || i == end_col {
                        "^"
                    } else {
                        "~"
                    })
                    .red()
                );
            }
            println!(" {}", self.msg.red());
        } else {
            println!("Error on line {}!", self.line);
            println!("{}", self.msg);
        }
    }
}
