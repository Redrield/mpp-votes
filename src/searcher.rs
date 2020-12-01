use common::Division;
use strsim::normalized_damerau_levenshtein;

pub fn search(haystack: &Vec<Division>, needle: &str) -> Vec<Division> {
    let mut best = vec![];
    for div in haystack {
        let mut score = normalized_damerau_levenshtein(&div.topic.to_lowercase(), &needle.to_lowercase());
        log::info!("Topic '{}', needle '{}', score {}", div.topic, needle, score);
        if div.topic.to_lowercase().contains(&needle.to_lowercase()) {
            score += 0.4;
            score = score.max(1.0);
        }

        if score >= 0.75 {
            best.push((score, div.clone()));
        }
    }

    best.sort_by(|(score1, _), (score2, _)| PartialOrd::partial_cmp(score1, score2).unwrap());
    best.into_iter().map(|(_, div)| div).collect()
}