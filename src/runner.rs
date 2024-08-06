use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => Ok(QueryResult::Added(TodoItem::new(
            Index::new(0),
            Description::new("foo"),
            vec![Tag::new("foo")],
            false,
        ))),
        Query::Done(idx) => Ok(QueryResult::Done),
        Query::Search(params) => {
            unimplemented!()
        }
    }
}
