use std::cmp::Ordering;

#[derive(Eq)]
pub struct Person {
    pub(crate) name: String,
    pub(crate) info: i64,
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.info.cmp(&other.info)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && self.name == other.name
    }
}
