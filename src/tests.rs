mod core {
    use crate::runner::Runner;
    use crate::util::{colored, GREEN};
    use crate::TodoList;

    use std::io;
    use std::io::prelude::*;
    use tokio::fs::File;

    #[tokio::test]
    async fn process_fixture() {
        File::create("./output_log.txt")
            .await
            .expect("Unable to open log output file");

        let mut tl: TodoList = TodoList::new();
        let input = include_bytes!("../tests/fixtures/sample.in");
        let text = String::from_utf8(input.to_vec()).unwrap();

        let mut runner = Runner {
            reader: io::stdin().lock(),
            writer: io::stdout(),
            log_path: Some(String::from("./output_log.txt")),
        };

        for (_, l) in text.lines().into_iter().enumerate() {
            write!(&mut runner.writer, "{}\n", colored(GREEN, l)).expect("Unable to write");

            runner.run_line(&l, &mut tl).await;
        }
    }
}
