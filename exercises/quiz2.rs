#[derive(Debug)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::borrow::Cow;
    use super::Command;

    pub fn transformer<'a>(input: &'a [(&'a str, Command)]) -> Vec<Cow<'a, str>> {
        input
            .iter()
            .map(|(inp, cmd)| match cmd {
                Command::Uppercase => Cow::Owned(inp.to_uppercase()),
                Command::Trim => Cow::Borrowed(inp.trim()),
                &Command::Append(num_times) => {
                    let mut new_s = inp.to_string();
                    (0..num_times).for_each(|_| new_s.push_str("bar"));
                    Cow::Owned(new_s)
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
        let input = vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ];
        let output = transformer(&input);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
