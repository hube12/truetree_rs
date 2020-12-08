use std::cmp::Ordering;
use core::fmt;
use truetree::AvlTree;

#[derive(Debug, Clone, Eq, Hash)]
pub struct Payload {
    age: u32,
    name: String,
}

impl PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age && self.name == other.name
    }
}

impl Ord for Payload {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}

impl PartialOrd for Payload {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"{}\":\"{}\"}}", self.age, self.name)
    }
}

#[test]
fn test_avl() {
    let test_payload_1: Payload = Payload { age: 42, name: "1".to_string() };
    let test_payload_2: Payload = Payload { age: 420, name: "2".to_string() };
    let test_payload_3: Payload = Payload { age: 66, name: "3".to_string() };
    let test_payload_4: Payload = Payload { age: 88, name: "4".to_string() };
    let test_payload_5: Payload = Payload { age: 99, name: "5".to_string() };
    let mut tree: AvlTree<Payload> = AvlTree::new();
    assert!(tree.insert(&test_payload_1).is_ok());
    assert!(tree.is_balanced());
    tree.insert(&test_payload_2).expect("Insert failed")
        .insert(&test_payload_3).expect("Insert failed")
        .insert(&test_payload_4).expect("Insert failed")
        .insert(&test_payload_5).expect("Insert failed");
    assert_eq!(tree.width(), 3);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.count(), 5);
    assert!(tree.is_balanced());
    assert_eq!(tree.height(), tree.depth());
    assert_eq!(tree.min().expect("No Min"), test_payload_1);
    assert_eq!(tree.max().expect("No Max"), test_payload_2);
    assert_eq!(tree.dump(false).expect("Missing print"), r#"[{"66":"3"},[{"42":"1"},null,null],[{"99":"5"},[{"88":"4"},null,null],[{"420":"2"},null,null]]]"#);
    assert_eq!(tree.dump(true).expect("Missing print"), r#"[
   {"66":"3"},
   [
      {"42":"1"},
      null,
      null
   ],
   [
      {"99":"5"},
      [
         {"88":"4"},
         null,
         null
      ],
      [
         {"420":"2"},
         null,
         null
      ]
   ]
]"#);
    let res = tree.remove(&test_payload_1);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().count(), 4);
    assert_eq!(tree.width(), 3);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.count(), 4);
    assert!(tree.is_balanced());
    assert_eq!(tree.height(), tree.depth());
    assert_eq!(tree.min().expect("No Min"), test_payload_3);
    // Try to delete 420 (we don't know the other field)
    let test_payload_6 = Payload {
        age: 420,
        name: String::new(),
    };
    let res = tree.get(&test_payload_6);
    assert!(tree.contains(&test_payload_6));
    assert!(!tree.contains_exact(&test_payload_6));
    // we got testpayload2 without knowing name
    assert!(res.is_some());
    assert_eq!(res.as_ref().unwrap(), &test_payload_2);
    let res = tree.remove(&res.as_ref().unwrap());
    assert!(res.is_ok());
    assert_eq!(tree.count(), 3);
    assert!(tree.contains(&test_payload_5));
    assert!(tree.contains_exact(&test_payload_5));
    assert_eq!(tree.max().expect("No max"), test_payload_5);
    tree.clear();
    assert!(tree.is_balanced());
    assert_eq!(tree.depth(), 0);
    tree.delete(); // you can not use the tree afterwards
}