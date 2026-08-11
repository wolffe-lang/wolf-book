//! Chapter 8 §8.4's contrast: the doubly-linked recency ring in Rust,
//! written the way the standard library asks for it. Compiled and run
//! by `cargo xtask contrast` so the printed program cannot rot.
//!
//! The book prints the `Doc`/`ring` pair verbatim and discusses the
//! third field of the deal — `nodes`, the `Vec` that has to outlive
//! every link, because the ring cannot hold itself up.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Doc {
    pub title: &'static str,
    pub newer: RefCell<Weak<Doc>>,
    pub older: RefCell<Weak<Doc>>,
}

pub fn ring(titles: &[&'static str]) -> Vec<Rc<Doc>> {
    let nodes: Vec<Rc<Doc>> = titles
        .iter()
        .map(|title| {
            Rc::new(Doc {
                title,
                newer: RefCell::new(Weak::new()),
                older: RefCell::new(Weak::new()),
            })
        })
        .collect();
    let n = nodes.len();
    for i in 0..n {
        *nodes[i].newer.borrow_mut() = Rc::downgrade(&nodes[(i + 1) % n]);
        *nodes[i].older.borrow_mut() = Rc::downgrade(&nodes[(i + n - 1) % n]);
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn walk_newer(start: &Rc<Doc>, steps: usize) -> Vec<&'static str> {
        let mut out = Vec::new();
        let mut cur = Rc::clone(start);
        for _ in 0..steps {
            out.push(cur.title);
            let next = cur.newer.borrow().upgrade().expect("the ring is held alive");
            cur = next;
        }
        out
    }

    #[test]
    fn the_ring_closes() {
        let nodes = ring(&["regions", "moves", "errors"]);
        assert_eq!(
            walk_newer(&nodes[0], 4),
            vec!["regions", "moves", "errors", "regions"]
        );
        let older = nodes[0].older.borrow().upgrade().unwrap();
        assert_eq!(older.title, "errors");
    }

    /// The claim §8.4 makes about the `Vec`: drop it and every link in
    /// the ring is already dead, because none of them is an owner.
    #[test]
    fn the_vec_is_the_only_owner() {
        let nodes = ring(&["regions", "moves"]);
        let kept = Rc::clone(&nodes[0]);
        drop(nodes);
        assert!(kept.newer.borrow().upgrade().is_none());
    }
}
