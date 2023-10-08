use std::collections::HashMap;
extern crate itertools;

use itertools::Itertools;

// in this program chars are syms
use Pattern::*;
#[derive(Clone, Debug)]
enum Pattern {
    Literal(String, char), // name, expr
    Sequence(String),
    NullSequence(String),
    Blank(String),
}

fn possible_lengths2(expr: &Vec<char>, pat: &Vec<Pattern>) -> Vec<Vec<usize>> {
    let mut lens = vec![];
    for (i, p) in pat.iter().enumerate() {
        let possible = match p {
            Pattern::Blank(_) | Pattern::Literal(_, _) => 1..=1,
            Pattern::Sequence(_) => 1..=expr.len(),
            Pattern::NullSequence(_) => 0..=expr.len(),
        };
        lens.push(possible.collect::<Vec<_>>())
    }
    lens
}
fn build_match_from_candidate(
    expr: &Vec<char>,
    pat: &Vec<Pattern>,
    candidate: &Vec<&usize>,
) -> Vec<(Pattern, Vec<char>)> {
    let mut result = vec![];
    let mut start = 0;

    for (pattern, &length) in pat.iter().zip(candidate.iter()) {
        let end = start + length;
        result.push((pattern.clone(), expr[start..end].to_vec()));
        start = end;
    }

    result
}

fn has_consistent_mappings(matches: &Vec<(Pattern, Vec<char>)>) -> bool {
    let mut mappings: HashMap<String, &Vec<char>> = HashMap::new();
    for (pattern, subseq) in matches.iter() {
        match pattern {
            Pattern::Literal(name, _)
            | Pattern::Sequence(name)
            | Pattern::NullSequence(name)
            | Pattern::Blank(name) => {
                if let Some(existing_subseq) = mappings.get(name) {
                    if existing_subseq != &subseq {
                        return false;
                    }
                } else {
                    mappings.insert(name.clone(), subseq);
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
    let expr = vec!['f', 'a', 'b', 'a', 'b'];
    let pattern = vec![
        Literal("f1".to_string(), 'g'),
        NullSequence("xs".to_string()),
        NullSequence("xs".to_string()),
        NullSequence("zs".to_string()),
        Blank("x".to_string())
    ];
    let lists = possible_lengths2(&expr, &pattern);
    println!("{lists:?}");

    let candidates = lists
        .iter()
        .multi_cartesian_product()
        .filter(|x| x.iter().map(|&&val| val).sum::<usize>() == expr.len());

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
