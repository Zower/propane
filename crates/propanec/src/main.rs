use codespan::Files;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

fn main() {
    let mut files = Files::new();

    let main = files.add(
        "main",
        r#"
            let main = 3 + 3;
            return main;
        "#,
    );

    let tokens = propane_lexer::tokenize(files.source(main));
    match propane_parser::parse(main, files.source(main), &tokens) {
        Ok(result) => {
            dbg!(result);
        }
        Err(errors) => {
            for error in errors {
                let writer = StandardStream::stderr(ColorChoice::Always);
                let config = codespan_reporting::term::Config::default();

                codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &error).unwrap();
            }

        }
    }
}
