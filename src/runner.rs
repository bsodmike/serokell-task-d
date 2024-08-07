use crate::*;
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

static LAZY_TODOITEM_MAP: LazyLock<Mutex<Option<HashMap<Index, TodoItem>>>> =
    LazyLock::new(|| Mutex::new(Some(HashMap::new())));
static LAZY_WORD_MAP: LazyLock<Mutex<Option<HashMap<String, Vec<Index>>>>> =
    LazyLock::new(|| Mutex::new(Some(HashMap::new())));
// FIXME: Optimize: Use char instead of string
static LAZY_CHARACTER_MAP: LazyLock<Mutex<Option<HashMap<String, Vec<Index>>>>> =
    LazyLock::new(|| Mutex::new(Some(HashMap::new())));

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

            let (item, _) = tl.push(desc, tags);
            {
                let index_lock = &mut *LAZY_TODOITEM_MAP.lock().await;
                if let Some(index_map) = index_lock {
                    index_map.insert(item.index, item.clone());
                    dbg!(index_map);
                };

                let index_lock = &mut *LAZY_WORD_MAP.lock().await;
                if let Some(index_map) = index_lock {
                    for word in words {
                        let collection = if let Some(has_word) = index_map.get(&word) {
                            let mut indices: Vec<Index> = has_word.to_vec().into_iter().collect();
                            indices.push(item.index);
                            indices
                        } else {
                            vec![item.index]
                        };

                        index_map.insert(word, collection);
                    }
                    dbg!(index_map);
                }
            }

            let result = QueryResult::Added(item);
            Ok(result)
        }
        Query::Done(idx) => Ok(QueryResult::Done),
        Query::Search(params) => {
            let tags = params.tags;
            let mut results: Vec<TodoItem> = vec![];
            let mut have_results = true;

            dbg!(&tl);

            for word in params.words {
                let index_lock = &mut *LAZY_WORD_MAP.lock().await;
                if let Some(index_map) = index_lock {
                    if let Some(ids) = index_map.get(&word.0) {
                        for id in ids {
                            let index_lock = &mut *LAZY_TODOITEM_MAP.lock().await;
                            if let Some(items) = index_lock {
                                if let Some(item) = items.get(id) {
                                    results.push(item.clone());
                                    have_results = true;
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
