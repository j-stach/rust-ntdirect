
use chrono::{DateTime, Utc};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(super) enum MarketDataType {
    Last,
    Ask,
    Bid,
}

#[allow(dead_code)]
pub(super) fn string_from_datetime(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y%m%d%H%M%S").to_string()
}

#[allow(dead_code)]
pub(super) fn separate(list: String) -> Vec<String> {
    let separator: char = '|';

    let mut separators = list.char_indices().filter(|(_, char)| char == &separator).map(|(index, _)| index);
    let separator_count: usize = separators.clone().count();
    let mut separated: Vec<String> = Vec::with_capacity(separator_count + 1);

    let mut start: usize = 0;
    while let Some(index) = separators.next() {
        let chunk: &str = &list[start..index];
        if !chunk.is_empty() {
            separated.push(chunk.to_string())
        }
        start = index + 1;
    }
    
    let last_chunk: &str = &list[start..];
    if !last_chunk.is_empty() {
        separated.push(last_chunk.to_string())
    }

    return separated
}