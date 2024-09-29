// https://leetcode.com/problems/all-oone-data-structure/description/
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    frequency: i32,
    keys: HashSet<String>,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(frequency: i32) -> Self {
        Self {
            frequency,
            keys: HashSet::new(),
            next: None,
            previous: None,
        }
    }

    fn move_key_up(rc_node: &mut Rc<RefCell<Node>>, key: &str) -> Rc<RefCell<Node>> {
        let next_freq = Self::next_node_freq(rc_node);
        let curr_freq = rc_node.borrow().frequency;
        if curr_freq + 1 != next_freq {
            Self::append(rc_node, curr_freq + 1);
        }

        let mut this = rc_node.borrow_mut();
        this.keys.remove(key);
        let mut next = this.next.as_mut().unwrap().borrow_mut();
        next.keys.insert(key.to_owned());
        drop(next);
        let next = this.next.as_ref().unwrap().clone();
        drop(this);
        Self::remove_if_empty(rc_node);
        next
    }

    fn move_key_down(rc_node: &mut Rc<RefCell<Node>>, key: &str) -> Rc<RefCell<Node>> {
        let previous_freq = Self::previous_node_freq(rc_node);
        let curr_freq = rc_node.borrow().frequency;
        if previous_freq + 1 != curr_freq {
            let mut previous_rc = rc_node
                .borrow_mut()
                .previous
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();
            Self::append(&mut previous_rc, curr_freq - 1)
        }

        let mut this = rc_node.borrow_mut();
        this.keys.remove(key);
        let previous = this.previous.as_mut().unwrap().upgrade().unwrap().clone();
        let mut prev_borrow = previous.borrow_mut();
        prev_borrow.keys.insert(key.to_owned());
        drop(prev_borrow);
        drop(this);
        Self::remove_if_empty(rc_node);
        previous
    }

    fn remove_if_empty(rc_node: &mut Rc<RefCell<Node>>) {
        let mut this = rc_node.borrow_mut();
        if !this.keys.is_empty() {
            return;
        }

        let previous = this.previous.as_mut().unwrap().upgrade().unwrap();
        let mut next = this.next.as_mut().unwrap().borrow_mut();
        next.previous = Some(Rc::downgrade(&previous));
        drop(next);
        previous.borrow_mut().next = this.next.take();
        drop(this);
    }

    fn append(rc_node: &mut Rc<RefCell<Node>>, frequency: i32) {
        let mut node_borrow = rc_node.borrow_mut();
        let next = node_borrow.next.take().unwrap();
        let new_node = Rc::new(RefCell::new(Node::new(frequency)));
        next.borrow_mut().previous = Some(Rc::downgrade(&new_node));
        let mut new_node_borrow = new_node.borrow_mut();
        new_node_borrow.next = Some(next.clone());
        new_node_borrow.previous = Some(Rc::downgrade(rc_node));
        node_borrow.next = Some(new_node.clone());
    }

    fn next_node_freq(rc_node: &Rc<RefCell<Node>>) -> i32 {
        rc_node.borrow().next.as_ref().unwrap().borrow().frequency
    }

    fn previous_node_freq(rc_node: &Rc<RefCell<Node>>) -> i32 {
        rc_node
            .borrow()
            .previous
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .frequency
    }
}

struct AllOne {
    frequencies: HashMap<String, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(Node::new(-1)));
        let tail = Rc::new(RefCell::new(Node::new(-1)));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().previous = Some(Rc::downgrade(&head));
        AllOne {
            frequencies: HashMap::new(),
            head,
            tail,
        }
    }

    fn inc(&mut self, key: String) {
        if let Some(node) = self.frequencies.get_mut(&key) {
            let new_home = Node::move_key_up(node, &key);
            self.frequencies.insert(key.clone(), new_home);
        } else {
            let freq_after_head = Node::next_node_freq(&self.head);
            if freq_after_head != 1 {
                Node::append(&mut self.head, 1);
            }
            let mut this = self.head.borrow_mut();
            let after_head_rc = this.next.as_mut().unwrap();
            let mut after_head = after_head_rc.borrow_mut();
            after_head.keys.insert(key.clone());
            drop(after_head);
            self.frequencies.insert(key, after_head_rc.clone());
        }
    }

    fn dec(&mut self, key: String) {
        let node = self.frequencies.get_mut(&key).unwrap();
        let curr_freq = node.borrow().frequency;
        if curr_freq == 1 {
            node.borrow_mut().keys.remove(&key);
            Node::remove_if_empty(node);
            self.frequencies.remove(&key);
            return;
        }

        let new_home = Node::move_key_down(node, &key);
        self.frequencies.insert(key, new_home);
    }

    fn get_max_key(&self) -> String {
        if !self.frequencies.is_empty() {
            let before_tail = self
                .tail
                .borrow_mut()
                .previous
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap();
            let before_borrow = before_tail.borrow();
            before_borrow.keys.iter().next().unwrap().to_string()
        } else {
            String::from("")
        }
    }

    fn get_min_key(&self) -> String {
        if !self.frequencies.is_empty() {
            let head = self.head.borrow_mut();
            let after_head = head.next.as_ref().unwrap();
            let after_head_borrow = after_head.borrow();
            after_head_borrow.keys.iter().next().unwrap().to_string()
        } else {
            String::from("")
        }
    }
}

fn main() {
    let test_cases = [
        (
            vec![
                "AllOne",
                "inc",
                "inc",
                "inc",
                "inc",
                "inc",
                "dec",
                "dec",
                "getMaxKey",
                "getMinKey",
            ],
            vec![
                vec![],
                vec!["a"],
                vec!["b"],
                vec!["b"],
                vec!["b"],
                vec!["b"],
                vec!["b"],
                vec!["b"],
                vec![],
                vec![],
            ],
        ),
        (
            vec![
                "AllOne",
                "inc",
                "inc",
                "inc",
                "inc",
                "inc",
                "inc",
                "dec",
                "dec",
                "getMinKey",
                "dec",
                "getMaxKey",
                "getMinKey",
            ],
            vec![
                vec![],
                vec!["a"],
                vec!["b"],
                vec!["b"],
                vec!["c"],
                vec!["c"],
                vec!["c"],
                vec!["b"],
                vec!["b"],
                vec![],
                vec!["a"],
                vec![],
                vec![],
            ],
        ),
    ];

    for (commands, args) in test_cases {
        println!("TEST CASE");
        for (command, arg) in commands.iter().zip(&args) {
            print!("{:?} {:?} | ", command, arg);
        }
        println!("");
        let mut allone = AllOne::new();
        for (command, arg) in commands.into_iter().zip(args) {
            match command {
                "inc" => allone.inc(arg.first().unwrap().to_string()),
                "dec" => allone.dec(arg.first().unwrap().to_string()),
                "getMaxKey" => println!("{}", allone.get_max_key()),
                "getMinKey" => println!("{}", allone.get_min_key()),
                _ => {}
            }
        }
    }
}
