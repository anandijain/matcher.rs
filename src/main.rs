use std::collections::HashMap;
extern crate itertools;
use itertools::Itertools;
use Pattern::*;

// in this program chars are syms
#[derive(Clone, Debug)]
enum Pattern {
    Literal(String, Expr),              // name, expr
    Sequence(String, Option<Expr>),     // name, head
    NullSequence(String, Option<Expr>), // name, head
    Blank(String, Option<Expr>),        // name, head
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Expr {
    Sym(String),
    List(Vec<Expr>),
}

fn sym(name: &str) -> Expr {
    Expr::Sym(name.to_string())
}

fn list(exprs: Vec<Expr>) -> Expr {
    Expr::List(exprs)
}

fn head(expr: &Expr) -> Expr {
    match expr {
        Expr::Sym(_) => sym("Sym"),
        Expr::List(list) => list[0].clone(),
    }
}

impl Expr {
    pub fn length(&self) -> usize {
        match self {
            Expr::Sym(_) => 0,
            Expr::List(lst) => lst.len().saturating_sub(1),
        }
    }
}

fn possible_lengths(expr: &Expr, pat: &Vec<Pattern>) -> Vec<Vec<usize>> {
    match expr {
        Expr::List(list) => {
            let mut lens = vec![];
            for (i, p) in pat.iter().enumerate() {
                let possible = match p {
                    Pattern::Blank(_, _) | Pattern::Literal(_, _) => 1..=1,
                    Pattern::Sequence(_, _) => 1..=list.len(),
                    Pattern::NullSequence(_, _) => 0..=list.len(),
                };
                lens.push(possible.collect::<Vec<_>>())
            }
            return lens;
        }
        _ => panic!("not implemented"),
    }
}
fn build_match_from_candidate(
    expr: &Expr,
    pat: &Vec<Pattern>,
    candidate: &Vec<&usize>,
) -> Vec<(Pattern, Expr)> {
    match expr {
        Expr::List(list) => {
            let mut result = vec![];
            let mut start = 0;

            for (pattern, &length) in pat.iter().zip(candidate.iter()) {
                let end = start + length;
                result.push((pattern.clone(), Expr::List(list[start..end].to_vec())));
                start = end;
            }

            return result;
        }
        Expr::Sym(name) => panic!(),
    }
}

fn has_consistent_mappings(matches: &Vec<(Pattern, Expr)>) -> bool {
    let mut mappings: HashMap<String, Expr> = HashMap::new();
    for (pattern, subseq) in matches.iter() {
        match pattern {
            Pattern::Literal(name, val) => {
                // a literal should only go to a single expr, despite being a List
                assert!(subseq.length() == 0);
                match subseq {
                    Expr::Sym(_) => panic!(),
                    Expr::List(ls) => {
                        if val != &ls[0] {
                            return false;
                        }
                    },
                }
            }
            Pattern::Sequence(name, head)
            | Pattern::NullSequence(name, head)
            | Pattern::Blank(name, head) => {
                if let Some(existing_subseq) = mappings.get(name) {
                    if existing_subseq != subseq {
                        return false;
                    }
                } else {
                    mappings.insert(name.clone(), subseq.clone());
                }
            }
        }
    }
    true
}

fn main() {
    // let expr = vec!['f', 'a', 'b', 'c', 'd', 'e'];
    // let pattern = vec![
    //     NullSequence("foo".to_string()),
    //     Literal("f1".to_string(), 'f'),
    //     Sequence("xs".to_string()),
    //     Sequence("ys".to_string()),
    //     Blank("x".to_string()),
    //     NullSequence("zs".to_string()),
    // ];
    let expr = Expr::List(vec![
        sym("f"),
        list(vec![sym("a"), sym("b")]),
        list(vec![sym("a"), sym("b")]),
        sym("c")
    ]);
    let pattern = vec![
        Literal("f1".to_string(), sym("g")),
        NullSequence("xs".to_string(), None),
        NullSequence("xs".to_string(), None),
        Blank("x".to_string(), None),
    ];
    let lists = possible_lengths(&expr, &pattern);
    println!("{lists:?}");

    let candidates = lists
        .iter()
        .multi_cartesian_product()
        .filter(|x| x.iter().map(|&&val| val).sum::<usize>() == expr.length() + 1);

    let filtered_candidates = candidates.clone().filter(|combination| {
        let matches = build_match_from_candidate(&expr, &pattern, combination);
        has_consistent_mappings(&matches)
    });
    // let ans =  filtered_candidates.next();
    // pretty sure its always the first candidate
    for combination in filtered_candidates.sorted() {
        println!("{:?}", combination);
        println!(
            "{:?}",
            build_match_from_candidate(&expr, &pattern, &combination)
        );
    }
}
