#[derive(Debug)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: &[(&str, Command)]) -> Vec<String> {
        input
            .iter()
            .map(|(inp, cmd)| match cmd {
                Command::Uppercase => inp.to_uppercase(),
                Command::Trim => inp.trim().to_string(),
                &Command::Append(num_times) => {
                    let mut new_s = inp.to_string();
                    (0..num_times).for_each(|_| new_s.push_str("bar"));
                    new_s
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(&vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
