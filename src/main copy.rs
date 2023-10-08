use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Pattern {
    L(char),   // Literal
    S(String), // Sequence
    N(String), // Null Sequence
    P(String), // Placeholder
}

#[derive(Debug, Clone, Default)]
struct Binding {
    bindings: HashMap<String, (usize, usize)>, // Represents a slice of the expression
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

            // Start with the longest match possible and work our way down
            for i in indices.clone().into_iter().rev() {
                if let Some(mut sub_result) = is_match(&expr[i..], &pattern[1..], cache) {
                    sub_result.bindings.insert(
                        placeholder.clone(),
                        (expr.len() - expr[i..].len(), expr.len()),
                    );

                    let potential_result = is_match(&expr[..expr.len() - i], pattern, cache);
                    if potential_result.is_some() {
                        return potential_result;
                    }

                    result = Some(sub_result);
                }
            }
        }

        _ => {}
    }

    cache.insert((expr.len(), pattern.len()), result.clone());
    result
}

fn main() {
    let expression = vec!['f', 'a', 'b', 'c'];
    let pattern = vec![
        Pattern::L('f'),
        Pattern::N("ys".to_string()),
        Pattern::S("xs".to_string()),
        Pattern::P("x".to_string()),
    ];

    let mut cache = HashMap::new();
    let match_result = is_match(&expression, &pattern, &mut cache);
    println!("Match: {:?}", match_result);
}
