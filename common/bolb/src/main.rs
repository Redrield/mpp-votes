use strsim::normalized_damerau_levenshtein;

fn main() {
    println!("{}", normalized_damerau_levenshtein("Second Reading of Bill 213, An Act to reduce burdens on people and businesses by enacting, amending and repealing various Acts and revoking a regulation.", "Bill 213 Second Reading"));
}
