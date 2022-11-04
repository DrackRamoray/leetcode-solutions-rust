use std::collections::{HashMap, HashSet};

pub struct ValidWordAbbr {
    data: HashMap<String, HashSet<String>>,
}

impl ValidWordAbbr {

    fn new(dictionary: Vec<String>) -> Self {
        let mut data = HashMap::new();

        for d in dictionary {
            if d.len() <= 2 {
                data.entry(d.clone()).or_insert(HashSet::new()).insert(d);
            } else {
                data.entry(format!("{}{}{}", &d[0..1], d.len() - 2, &d[d.len()-1..])).or_insert(HashSet::new()).insert(d);
            }
        }

        Self { data }
    }

    fn is_unique(&self, word: String) -> bool {
        let w = if word.len() <= 2 {
            word.clone()
        } else {
            format!("{}{}{}", &word[0..1], word.len() - 2, &word[word.len()-1..])
        };

        if let Some(d) = self.data.get(&w) {
            d.contains(&word) && d.len() == 1
        } else {
            true
        }
    }
}

#[test]
fn it_works() {
    let obj = ValidWordAbbr::new(assist::vec_stringify!(vec!["deer", "door", "cake", "card"]));
    assert_eq!(obj.is_unique("dear".to_string()), false);
    assert_eq!(obj.is_unique("cart".to_string()), true);
    assert_eq!(obj.is_unique("cane".to_string()), false);
    assert_eq!(obj.is_unique("make".to_string()), true);
    assert_eq!(obj.is_unique("door".to_string()), false);
}
