use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(val, list)| {
            list.iter()
                .map(char::to_ascii_lowercase)
                .zip(std::iter::repeat(*val))
        })
        .collect::<BTreeMap<char, i32>>()
}
