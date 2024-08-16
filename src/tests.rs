mod core {
    use crate::runner::Runner;
    use crate::util::{colored, GREEN};
    use crate::TodoList;

    use fst::automaton::Subsequence;
    use fst::{IntoStreamer, Set};
    use std::io;
    use std::io::prelude::*;
    use tokio::fs::File;

    const OUTPUT_FILE: &str = "./tests/fixtures/sample.out";

    #[tokio::test]
    async fn process_fixture() {
        File::create(OUTPUT_FILE)
            .await
            .expect("Unable to open log output file");

        let mut tl: TodoList = TodoList::new();
        let input = include_bytes!("../tests/fixtures/sample.in");
        let text = String::from_utf8(input.to_vec()).unwrap();

        let mut runner = Runner {
            reader: io::stdin().lock(),
            writer: io::stdout(),
            log_path: Some(String::from(OUTPUT_FILE)),
        };

        for (_, l) in text.lines().into_iter().enumerate() {
            write!(&mut runner.writer, "{}\n", colored(GREEN, l)).expect("Unable to write");

            runner.run_line(&l, &mut tl).await;
        }
    }

    // This crate provides a separate challenge, in the sense building the Set needs lexicographically
    // ordered set of byte strings; the ordering poses a challenge as the `Set` cannot be manipulated
    // in memory once created.
    //
    // It is unlikely the `fst` crate could be used for this application and instead we should use
    // `BTreeMap` or `HashMap` instead.
    //
    // However, an expensive approach is to pin the Set in memory, and on each evaluation, words are
    // sorted and a new set is created and replaced, then used for the subsequence search.
    #[test]
    fn using_fst_crate() {
        unimplemented!();

        // A convenient way to create sets in memory.
        let keys = vec!["buy bread", "buy milk", "call parents"];
        let set: Set<Vec<u8>> = Set::from_iter(keys).expect("Unable to create Set");

        // Build our fuzzy query.
        let subseq = Subsequence::new("a");

        // Apply our fuzzy query to the set we built.
        let stream = set.search(subseq).into_stream();

        let keys = stream
            .into_strs()
            .expect("Unable to convert stream into vector");
        assert_eq!(keys, vec!["buy bread", "call parents"]);
    }
}
