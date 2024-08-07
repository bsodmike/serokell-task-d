use crate::*;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use util::{colored, YELLOW};

use std::io::{BufRead, Write};

#[derive(Debug)]
pub struct Runner<R, W> {
    pub reader: R,
    pub writer: W,
    pub log_path: Option<String>,
}

impl<R, W> Runner<R, W>
where
    R: std::fmt::Debug + BufRead,
    W: std::fmt::Debug + Write,
{
    pub async fn run_line(&mut self, line: &str, tl: &mut TodoList) {
        if let Ok((_, q)) = parser::query(line) {
            match run_query(self, q, tl) {
                Ok(r) => {
                    println!("{}", colored(YELLOW, r.to_string().as_str()));

                    if let Some(ref log_path) = self.log_path {
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(log_path)
                            .await
                            .expect("Unable to open log file");
                        file.write_all(format!("{}\n", r).as_bytes())
                            .await
                            .expect("Unable to write to file");
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

pub(crate) fn run_query<R, W>(
    runner: &mut Runner<R, W>,
    q: Query,
    tl: &mut TodoList,
) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let item = tl.push(desc, tags);

            let result = QueryResult::Added(item);
            Ok(result)
        }
        Query::Done(idx) => Ok(QueryResult::Done),
        Query::Search(params) => {
            // "2 item(s) found";
            dbg!(params);

            let item = TodoItem::new(
                Index::new(0),
                Description::new("foo"),
                vec![Tag::new("tag")],
                false,
            );
            let results = vec![item];
            let result = QueryResult::Found(results);

            Ok(result)
        }
    }
}
