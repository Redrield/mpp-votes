use crate::votes::RawDivision;
use select::node::Node;

enum ParserState {
    CollectingTopic,
    CollectingAyes,
    CollectingNays,
    Discarding,
    Check,
    End,
}

/// Roughly extracts the important content out of DOM nodes that have already been found to contain a vote
///
/// 1. Go through the list of nodes from the start, collecting relevant nodes as text in the topic paragraph
/// This stops when there is a node that _seems_ to mark the start of irrelevant information. The transcripts _seem_
/// relatively consistent about this but the way that it's determined _really_ isn't good and could lead to redundant information in the topic
/// Once the AYES header has been read, append paragraphs that do not repeat the header to the raw_ayes (Repeated header for near unanimous motions)
/// Once the NAYS header has been read, append paragraphs that do not repeat the header to raw_nays
/// Once AYES and NAYS have not been seen in the node immediately following a list of names, the topic is assumed to be concluded (Assuming the end of the iterator isn't reached before then)
pub fn parse_division(division: &Vec<Node>, date: &str) -> RawDivision {
    let mut topic = String::new();
    let mut raw_ayes = String::new();
    let mut raw_nays = String::new();
    let mut state = ParserState::CollectingTopic;

    for node in division {
        match state {
            ParserState::CollectingTopic => {
                let lower = node.text().to_lowercase();
                if !lower.contains("debate") && !lower.contains("the following division") {
                    topic.push_str(&node.text());
                } else {
                    // Just in case it overflows
                    if node.text().contains("AYES") {
                        state = ParserState::CollectingAyes;
                    } else {
                        state = ParserState::Discarding;
                    }
                }
            }
            ParserState::CollectingAyes => {
                let t = node.text();
                if t.contains("AYES") {
                    continue;
                } else if t.contains("NAYS") {
                    state = ParserState::CollectingNays;
                    continue;
                } else {
                    raw_ayes.push_str(&t);
                }
            }
            ParserState::CollectingNays => {
                raw_nays.push_str(&node.text());
                state = ParserState::Check;
            }
            ParserState::Check => {
                if node.text().contains("NAYS") {
                    state = ParserState::CollectingNays;
                    continue; // NAYS continued
                } else {
                    state = ParserState::End;
                }
            }
            ParserState::Discarding => {
                let text = node.text();
                if text.contains("AYES") {
                    state = ParserState::CollectingAyes;
                } else if text.contains("NAYS") {
                    state = ParserState::CollectingNays;
                }
            }
            ParserState::End => break,
        }
    }

    RawDivision {
        date: date.to_string(),
        topic: topic.trim().to_string(),
        raw_ayes: raw_ayes.trim().to_string(),
        raw_nays: raw_nays.trim().to_string(),
    }
}
