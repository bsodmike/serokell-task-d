use std::{fmt, ops::Add};

use fst::Set;
use runner::{LAZY_DESC_MAP, LAZY_SET, LAZY_TODOITEM_MAP};

use crate::*;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Add for Index {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value() + other.value())
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}, {:?}", self.index, self.description, self.tags)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: vec![],
        }
    }

    pub async fn push(&mut self, description: Description, tags: Vec<Tag>) -> (TodoItem, Index) {
        let new_index = self.top_index + Index::new(1);

        let item = TodoItem::new(self.top_index, description, tags, false);
        let items = &mut self.items;

        // Update top_index
        self.top_index = new_index;

        // NOTE: Cloning
        items.push(item.clone());

        {
            let index_lock = &mut *LAZY_TODOITEM_MAP.lock().await;
            if let Some(index_map) = index_lock {
                index_map.insert(item.index, item.clone());
                dbg!(index_map);
            };

            let index_lock = &mut *LAZY_DESC_MAP.lock().await;
            if let Some(hsh) = index_lock {
                hsh.insert(item.description.0.clone(), item.index);

                let mut words = hsh.keys().into_iter().collect::<Vec<&String>>();
                words.sort();

                let index_lock = &mut *LAZY_SET.lock().await;
                if let Some(index_map) = index_lock {
                    let keys = words;
                    let set: Set<Vec<u8>> = Set::from_iter(keys).expect("Unable to create Set");

                    *index_map = set
                }
            };
        }

        (item, new_index)
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        unimplemented!();
    }

    pub fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        unimplemented!();
    }
}
