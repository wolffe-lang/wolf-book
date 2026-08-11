//! Chapter 16 §16.3's contrast: the same shelf, shared three ways in
//! Rust. Compiled with warnings denied and run by `cargo xtask
//! contrast`, so the programs the book prints beside wolf's are
//! programs that build and pass their own assertions.
//!
//! The book prints `shared_mutable` and `shared_frozen` verbatim and
//! discusses `moved_graph`, which is the shape a cyclic graph has to
//! take in Rust before it can change owners at all: nodes in a `Vec`,
//! edges as indices, the arena rebuilt by hand.

use std::sync::{Arc, Mutex};
use std::thread;

pub struct Doc {
    pub title: &'static str,
    pub words: u32,
}

/// Share one mutable shelf between two readers: an `Arc` for the
/// lifetime, a `Mutex` for the exclusion, and a lock on every read.
pub fn shared_mutable(docs: Vec<Doc>) -> u32 {
    let shelf = Arc::new(Mutex::new(docs));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let shelf = Arc::clone(&shelf);
        handles.push(thread::spawn(move || {
            let guard = shelf.lock().expect("the shelf's lock is poisoned");
            guard.iter().map(|d| d.words).sum::<u32>()
        }));
    }
    handles
        .into_iter()
        .map(|h| h.join().expect("a reader panicked"))
        .sum()
}

/// Share one immutable shelf between two readers: no lock, because the
/// type says nobody writes. This is the spelling wolf's `freeze` is.
pub fn shared_frozen(docs: Vec<Doc>) -> u32 {
    let shelf = Arc::new(docs);
    let mut handles = Vec::new();
    for _ in 0..2 {
        let shelf = Arc::clone(&shelf);
        handles.push(thread::spawn(move || {
            shelf.iter().map(|d| d.words).sum::<u32>()
        }));
    }
    handles
        .into_iter()
        .map(|h| h.join().expect("a reader panicked"))
        .sum()
}

/// Hand a *cyclic* recency ring to another thread. The ring cannot be
/// made of references, so it is made of indices into a `Vec` — the
/// arena, hand-rolled — and the whole arena moves into the closure.
pub struct Node {
    pub title: &'static str,
    pub newer: usize,
    pub older: usize,
}

pub fn moved_graph(titles: Vec<&'static str>) -> String {
    let n = titles.len();
    let nodes: Vec<Node> = titles
        .iter()
        .enumerate()
        .map(|(i, title)| Node {
            title,
            newer: (i + 1) % n,
            older: (i + n - 1) % n,
        })
        .collect();
    thread::spawn(move || {
        let mut cur = 0usize;
        let mut out = String::new();
        for _ in 0..=n {
            out.push_str(nodes[cur].title);
            out.push(' ');
            cur = nodes[cur].newer;
        }
        out
    })
    .join()
    .expect("the walker panicked")
}

fn shelf() -> Vec<Doc> {
    vec![
        Doc {
            title: "regions",
            words: 900,
        },
        Doc {
            title: "moves",
            words: 640,
        },
    ]
}

#[test]
fn both_readers_see_the_whole_shelf() {
    assert_eq!(shared_mutable(shelf()), 3080);
    assert_eq!(shared_frozen(shelf()), 3080);
}

#[test]
fn the_ring_walks_after_it_moves() {
    assert_eq!(
        moved_graph(vec!["regions", "moves", "errors"]),
        "regions moves errors regions "
    );
}
