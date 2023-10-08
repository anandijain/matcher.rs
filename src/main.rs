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
        Expr::List(list) => {
            // println!("expr: {:?}", expr);
            list[0].clone()
        }
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
) -> Vec<(Pattern, Vec<Expr>)> {
    match expr {
        Expr::List(list) => {
            let mut result = vec![];
            let mut start = 0;

            for (pattern, &length) in pat.iter().zip(candidate.iter()) {
                let end = start + length;
                result.push((pattern.clone(), list[start..end].to_vec()));
                start = end;
            }

            return result;
        }
        Expr::Sym(name) => panic!(),
    }
}

// fn has_consistent_mappings(matches: &Vec<(Pattern, Expr)>) -> bool {
fn has_consistent_mappings(matches: &Vec<(Pattern, Vec<Expr>)>) -> bool {
    let mut mappings: HashMap<String, Vec<Expr>> = HashMap::new();
    for (pattern, subseq) in matches.iter() {
        match pattern {
            Pattern::Literal(name, val) => {
                assert!(subseq.len() == 1);
                if val != &subseq[0] {
                    return false;
                }
            }
            Pattern::Sequence(name, p_head) => {
                if let Some(h) = p_head {
                    for s in subseq {
                        let s_head = head(s);
                        if s_head != *h {
                            return false;
                        }
                    }
                }
                if let Some(existing_subseq) = mappings.get(name) {
                    if existing_subseq != subseq {
                        return false;
                    }
                } else {
                    mappings.insert(name.clone(), subseq.clone());
                }
            }
            Pattern::NullSequence(name, p_head) => {
                if let Some(h) = p_head {
                    for s in subseq {
                        let s_head = head(s);
                        if s_head != *h {
                            return false;
                        }
                    }
                }
                if let Some(existing_subseq) = mappings.get(name) {
                    if existing_subseq != subseq {
                        return false;
                    }
                } else {
                    mappings.insert(name.clone(), subseq.clone());
                }
            }
            Pattern::Blank(name, p_head) => {
                if let Some(h) = p_head {
                    assert!(subseq.len() == 1);
                    let s_head = head(&subseq[0]);
                    println!("s_head {:?}", s_head);
                    if s_head != *h {
                        return false;
                    }
                }
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
        list(vec![sym("c"), sym("d")]),
    ]);
    let pattern = vec![
        Literal("f1".to_string(), sym("f")),
        NullSequence("xs".to_string(), Some(sym("a"))),
        NullSequence("xs".to_string(), Some(sym("a"))),
        Blank("x".to_string(), Some(sym("c"))),
    ];
    let lists = possible_lengths(&expr, &pattern);
    println!("{lists:?}");
    // println!("head  {:?}", head(&sym("c")));

    let candidates = lists
        .iter()
        .multi_cartesian_product()
        .filter(|x| x.iter().map(|&&val| val).sum::<usize>() == expr.length() + 1);
    // println!("{candidates:?}");

    let filtered_candidates = candidates.clone().filter(|combination| {
        let matches = build_match_from_candidate(&expr, &pattern, combination);
        // println!("{:?}", matches.iter().enumerate().collect::<Vec<_>>());
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
