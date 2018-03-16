use std::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use cfg;

pub struct Generator {
    left: bool,
    rules: HashMap<cfg::Symbol, Vec<Vec<cfg::Symbol>>>,
    queue: VecDeque<Vec<cfg::Symbol>>,
    min_len: usize,
    max_len: usize,
}

#[derive(Debug)]
pub struct GeneratedSet(pub HashSet<Vec<cfg::Symbol>>);

impl fmt::Display for GeneratedSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.0 {
            write!(
                f,
                "{}\n",
                item.iter()
                    .fold(String::new(), |acc, ref arg| acc + arg.to_string().as_ref())
            ).unwrap();
        }
        Ok(())
    }
}

impl Generator {
    pub fn new(grammar: cfg::CFG, lmin: u32, lmax: u32, left: bool) -> Generator {
        let mut rules: HashMap<cfg::Symbol, Vec<Vec<cfg::Symbol>>> = HashMap::new();
        for rule in grammar.simplify().productions {
            let mut symbols = match rules.get(&cfg::Symbol::N(rule.left.clone())) {
                Some(s) => s.clone(),
                None => Vec::new(),
            };
            symbols.push(rule.right.clone());
            rules.insert(cfg::Symbol::N(rule.left.clone()), symbols);
        }
        let mut queue = VecDeque::new();
        for cases in rules.get(&cfg::Symbol::N(grammar.start)) {
            for case in cases {
                queue.push_back(case.clone());
            }
        }
        Generator {
            left: left,
            rules: rules,
            queue: queue,
            min_len: lmin as usize,
            max_len: lmax as usize,
        }
    }
}

impl Iterator for Generator {
    type Item = Vec<cfg::Symbol>;

    fn next(&mut self) -> Option<Vec<cfg::Symbol>> {
        while let Some(next_item) = self.queue.pop_front() {
            if next_item.is_empty() {
                return Some(next_item);
            }
            if next_item.len() > self.max_len {
                // too long a sequence, drop it
                continue;
            }
            if next_item.iter().all(|x| x.is_terminal()) {
                // only terminals
                if next_item.len() >= self.min_len {
                    return Some(next_item);
                } else {
                    // too short a sequence, drop
                    continue;
                }
            }
            let idx = if self.left {
                next_item.iter().position(|x| x.is_nonterminal()).unwrap()
            } else {
                next_item.iter().rposition(|x| x.is_nonterminal()).unwrap()
            };
            if let Some(rules) = self.rules.get(&next_item[idx]) {
                for seq in rules {
                    let mut new_seq = next_item[..idx].to_vec();
                    new_seq.extend(seq.clone());
                    if next_item.len() > idx + 1 {
                        new_seq.extend(next_item[idx + 1..].to_vec());
                    }
                    self.queue.push_back(new_seq);
                }
            } else {
                unreachable!() // unreachable Nonterminal symbol ???
            }
        }
        None
    }
}