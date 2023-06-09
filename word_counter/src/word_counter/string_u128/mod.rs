use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct StringU128{
    leader: String,
    ct: u128,
}

impl PartialEq for StringU128 {
    fn eq(&self, other: &Self) -> bool {
        self.leader == other.leader && self.ct == other.ct
    }
}

impl PartialOrd for StringU128 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ct == other.ct {
            Some(self.leader.cmp(&other.leader))
        }
        else{
            Some(self.ct.cmp(&other.ct))
        }
    }
}

impl Ord for StringU128 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ct.cmp(&other.ct)
    }
}

impl StringU128 {
    pub fn new(s: &String, c: &u128) -> Self {
        StringU128 {
            leader: s.clone(),
            ct: *c,
        }
    }

    pub fn get_string(&self) -> String {
        self.leader.clone()
    }

    pub fn get_count(&self) -> u128 {
        self.ct
    }
}
