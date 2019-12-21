use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut r: BTreeMap<char, i32> = BTreeMap::new();
    h.iter().for_each(|(value, list)| {
        list.iter().for_each(|c| {
            r.insert(c.to_ascii_lowercase(), *value);
        });
    });
    r
}
