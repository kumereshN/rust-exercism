use std::collections::HashSet;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.



pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {

    let hset: HashSet<_> = hands
        .iter()
        .fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

    if hset.len() == 1 {
        hands.to_vec()
    } else {
        hands.to_vec()
    }
    // todo!("Out of {hands:?}, which hand wins?")
}
