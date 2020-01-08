use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_insert_with(|| vec![])
            .push(String::from(student))
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect::<Vec<u32>>()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).cloned().map(|mut names| {
            names.sort();
            names
        })
    }
}
