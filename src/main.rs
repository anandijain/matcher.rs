use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Pattern {
    L(char),   // Literal
    S(String), // Sequence
    N(String), // Null Sequence
    P(String), // Placeholder
}

#[derive(Debug, Clone, Default)]
struct Binding {
    bindings: BTreeMap<String, VecDeque<char>>,
}

fn is_match(
    expr: &[char],
    pattern: &[Pattern],
    cache: &mut HashMap<(usize, usize), Option<Binding>>,
) -> Option<Binding> {
    if let Some(ref cached) = cache.get(&(expr.len(), pattern.len())) {
        return cached.clone().clone();
    }

    if pattern.is_empty() {
        return if expr.is_empty() {
            Some(Binding::default())
        } else {
            None
        };
    }

    let mut result = None;

    match &pattern[0] {
        Pattern::L(literal) if !expr.is_empty() && expr[0] == *literal => {
            result = is_match(&expr[1..], &pattern[1..], cache);
        }
        Pattern::P(placeholder) | Pattern::S(placeholder) | Pattern::N(placeholder)
            if !expr.is_empty() =>
        {
            let indices: Vec<usize> = match &pattern[0] {
                Pattern::P(_) => (1..2).collect(),
                Pattern::S(_) => (1..=expr.len()).collect(),
                Pattern::N(_) => (0..=expr.len()).collect(),
                _ => unreachable!(),
            };

            for i in indices {
                if let Some(existing_binding) = result
                    .as_ref()
                    .and_then(|res| res.bindings.get(placeholder))
                {
                    let matched_seq: Vec<char> = expr[0..i].to_vec();
                    let existing_seq: Vec<char> = existing_binding.iter().cloned().collect();
                    if matched_seq != existing_seq {
                        continue;
                    }
                }

                if let Some(mut sub_result) = is_match(&expr[i..], &pattern[1..], cache) {
                    for ch in &expr[0..i] {
                        sub_result
                            .bindings
                            .entry(placeholder.clone())
                            .or_insert_with(VecDeque::new)
                            .push_front(*ch);
                    }
                    result = Some(sub_result);
                    break;
                }
            }
        }
        _ => {}
    }

    cache.insert((expr.len(), pattern.len()), result.clone());
    result
}

fn main() {
    let expression = vec!['a', 'b', 'a', 'b', 'c', 'd', 'e'];
    let pattern = vec![
        Pattern::S("xs".to_string()),
        Pattern::S("xs".to_string()),
        Pattern::N("ys".to_string()),
        Pattern::P("x".to_string()),
    ];

    let mut cache = HashMap::new();

    let match_result = is_match(&expression, &pattern, &mut cache);
    println!("cache: {:#?}", cache);
    println!("Match: {:?}", match_result);
}
