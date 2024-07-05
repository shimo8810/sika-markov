use once_cell::sync::Lazy;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::collections::HashMap;

type States = HashMap<i32, Vec<(i32, f64)>>;
type Symbols = HashMap<i32, char>;

static STATES: Lazy<States> = Lazy::new(|| {
    HashMap::from([
        (0, vec![(1, 1.0)]),                       // S
        (1, vec![(2, 0.5), (5, 0.5)]),             // し
        (2, vec![(3, 1.0)]),                       // か
        (3, vec![(4, 1.0)]),                       // の
        (4, vec![(3, 0.5), (4, 0.25), (1, 0.25)]), // こ
        (5, vec![(6, 1.0)]),                       // た
        (6, vec![(5, 0.5), (-1, 0.5)]),            // ん
        (-1, vec![]),                              // E
    ])
});

static SYMBOLS: Lazy<Symbols> = Lazy::new(|| {
    HashMap::from([
        (0, '_'),
        (1, 'し'),
        (2, 'か'),
        (3, 'の'),
        (4, 'こ'),
        (5, 'た'),
        (6, 'ん'),
        (-1, '_'),
    ])
});

fn markov_chain(s: i32) -> i32 {
    STATES[&s][WeightedIndex::new(STATES[&s].iter().map(|(_, p)| p))
        .unwrap()
        .sample(&mut thread_rng())]
    .0
}

fn sika_dfs() -> String {
    let mut t = String::new();

    fn dfs(s: i32, t: &mut String) {
        if s == -1 {
            return;
        }
        if s != 0 {
            t.push(SYMBOLS[&s]);
        }
        dfs(markov_chain(s), t);
    }

    dfs(0, &mut t);

    t
}

fn main() {
    println!("{}", sika_dfs());
}
