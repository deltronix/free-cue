use relm4::typed_view::list::RelmListItem;
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CueNumber(pub Option<usize>, pub Option<usize>);

pub struct CueNumberWidgets {
    button: gtk::Button,
    entry: gtk::Entry,
}
impl CueNumber {
    pub fn new() -> CueNumber {
        Self(None, None)
    }
    pub fn to_string(&self) -> String {
        match self.0 {
            None => "_".to_string(),
            Some(n) => format!("{}", n),
        }
    }
}

impl PartialOrd for CueNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CueNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.0, other.0) {
            (None, None) => match (self.1, other.1) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(&b),
            },
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a), Some(b)) => match a.cmp(&b) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => match (self.1, other.1) {
                    (None, None) => Ordering::Equal,
                    (None, Some(_)) => Ordering::Greater,
                    (Some(_), None) => Ordering::Less,
                    (Some(a), Some(b)) => a.cmp(&b),
                },
                Ordering::Greater => Ordering::Greater,
            },
        }
    }
}
