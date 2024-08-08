use crate::*;
use fst::automaton::Subsequence;
use fst::{IntoStreamer, Set};
use tokio::io::AsyncWriteExt;
use tokio::{fs::OpenOptions, sync::Mutex};
use util::{colored, YELLOW};

use std::any::Any;
use std::hash::Hash;
use std::{
    collections::HashMap,
    io::{BufRead, Write},
    sync::LazyLock,
};

pub(crate) mod storage;

pub static LAZY_TODOITEM_MAP: LazyLock<Mutex<Option<HashMap<Index, TodoItem>>>> =
    LazyLock::new(|| Mutex::new(Some(HashMap::new())));
pub static LAZY_DESC_MAP: LazyLock<Mutex<Option<HashMap<String, Index>>>> =
    LazyLock::new(|| Mutex::new(Some(HashMap::new())));
pub static LAZY_SET: LazyLock<Mutex<Option<Set<Vec<u8>>>>> = LazyLock::new(|| {
    Mutex::new(Some(
        Set::from_iter(vec![""]).expect("Unable to create set"),
    ))
});

// static LAZY_WORD_MAP: LazyLock<Mutex<Option<HashMap<String, Vec<Index>>>>> =
//     LazyLock::new(|| Mutex::new(Some(HashMap::new())));
// // FIXME: Optimize: Use char instead of string
// static LAZY_CHARACTER_MAP: LazyLock<Mutex<Option<HashMap<String, Vec<Index>>>>> =
//     LazyLock::new(|| Mutex::new(Some(HashMap::new())));

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
            match run_query(self, q, tl).await {
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

pub(crate) async fn run_query<R, W>(
    runner: &mut Runner<R, W>,
    q: Query,
    tl: &mut TodoList,
) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let description = desc.clone().to_string();
            let words: Vec<String> = description
                .split(" ")
                .into_iter()
                .map(|el| el.to_string())
                .collect();

            let (item, _) = tl.push(desc, tags).await;

            let result = QueryResult::Added(item);
            Ok(result)
        }
        Query::Done(idx) => Ok(QueryResult::Done),
        Query::Search(params) => {
            let tags = params.tags;
            let mut results: Vec<TodoItem> = vec![];

            dbg!(&tl);

            let index_lock = &mut *LAZY_SET.lock().await;
            if let Some(set) = index_lock {
                for word in params.words {
                    // Build fuzzy query.
                    let subseq = Subsequence::new(word.0.as_str());
                    // Apply fuzzy query to the set we built.
                    let stream = set.search(subseq).into_stream();

                    let keys = stream
                        .into_strs()
                        .expect("Unable to convert fst Stream into vector");

                    dbg!(&keys);

                    for key in keys {
                        let index_lock = &mut *LAZY_DESC_MAP.lock().await;
                        if let Some(hsh) = index_lock {
                            if let Some(index) = hsh.get(&key) {
                                let index_lock = &mut *LAZY_TODOITEM_MAP.lock().await;
                                if let Some(items) = index_lock {
                                    if let Some(item) = items.get(index) {
                                        results.push(item.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Ok(QueryResult::Found(results))
        }
    }
}
