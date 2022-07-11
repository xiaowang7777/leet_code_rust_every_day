use std::cell::RefCell;
use std::collections::HashMap;

struct MagicDictionary {
    data: RefCell<Vec<String>>,

    refe: RefCell<HashMap<String, Vec<u8>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            data: RefCell::new(Vec::new()),
            refe: RefCell::new(HashMap::new()),
        }
    }

    fn build_dict(&self, mut dictionary: Vec<String>) {
        let len = dictionary.len();
        for i in 0..len {
            let str = dictionary.get(i).cloned().unwrap();
            let bytes = str.as_bytes().to_vec();
            self.refe.borrow_mut().insert(str, bytes);
        }
        self.data.borrow_mut().append(&mut dictionary)
    }

    fn search(&self, search_word: String) -> bool {
        let re = self.refe.borrow();
        let search_fn = |str: &String| -> bool {
            let find = search_word.as_bytes().to_vec();
            let target = re.get(str).unwrap();

            let len = find.len();
            let mut found = 0;

            for i in 0..len {
                if *find.get(i).unwrap() != *target.get(i).unwrap() {
                    found += 1;
                }
                if found > 1 {
                    return false;
                }
            }

            found == 1
        };

        let data = &*self.data.borrow();
        let len = data.len();
        for i in 0..len {
            if data.get(i).unwrap().len() == search_word.len() && search_fn(data.get(i).unwrap()) {
                return true;
            }
        }

        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
mod test {
    use crate::_2022_07_11::MagicDictionary;

    #[test]
    fn test() {
        let obj = MagicDictionary::new();
        obj.build_dict(vec![String::from("hello"), String::from("leetcode")]);
        assert_eq!(obj.search(String::from("hello")), false);
        assert_eq!(obj.search(String::from("hhllo")), true);
        assert_eq!(obj.search(String::from("hell")), false);
        assert_eq!(obj.search(String::from("leetcoded")), false);
    }
}