use std::collections::HashMap;

pub struct Crate {
    id: char,
}

impl Crate {
    pub fn get_id(&self) -> char {
        self.id
    }
}

pub struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    pub fn get_top(&self) -> char {
        self.crates.last().unwrap().get_id()
    }

    pub fn from(crate_ids: &str) -> Self {
        let mut crates = Vec::new();

        for id in crate_ids.chars() {
            crates.push(Crate { id });
        }

        Stack { crates }
    }

    pub fn len(&self) -> i32 {
        self.crates.len() as i32
    }

    pub fn pop(&mut self) -> Crate {
        self.crates.pop().unwrap()
    }

    pub fn push(&mut self, c: Crate) {
        self.crates.push(c);
    }
}

#[cfg(test)]
mod stack_test {
    use super::*;

    #[test]
    fn from() {
        let s = Stack::from("ABC");
        assert_eq!(s.len(), 3);

        let s = Stack::from("");
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn pop() {
        let mut s = Stack::from("ABC");
        let popped = s.pop();
        assert_eq!(popped.get_id(), 'C');

        s.pop();
        s.pop();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn push() {
        let mut s = Stack::from("ABC");
        s.push(Crate { id: 'D' });
        assert_eq!(s.len(), 4);

        assert_eq!(s.pop().get_id(), 'D');
    }
}

pub struct StackCollection {
    stacks: HashMap<i32, Stack>,
}

impl StackCollection {
    pub fn from(stacks: Vec<Stack>) -> StackCollection {
        let mut stack_hashmap: HashMap<i32, Stack> = HashMap::new();

        let mut key = 1;
        for stack in stacks {
            stack_hashmap.insert(key, stack);
            key += 1;
        }

        StackCollection {
            stacks: stack_hashmap,
        }
    }

    fn get_stack(&mut self, key: i32) -> &mut Stack {
        self.stacks
            .get_mut(&key)
            .expect(format!("{} is not a valid key", key).as_str())
    }

    pub fn get_top_row(&self) -> String {
        let mut output = String::new();
        for i in 1..=9 {
            let stack = self.stacks.get(&i).unwrap();
            output.push(stack.get_top());
        }

        output
    }

    pub fn move_between_stacks(&mut self, amount: i32, from_stack: i32, to_stack: i32) {
        let mut popped_vec = Vec::new();
        for _ in 0..amount {
            let from = self.get_stack(from_stack);
            popped_vec.push(from.pop());
        }
        popped_vec.reverse();
        let to = self.get_stack(to_stack);

        for s in popped_vec {
            to.push(s);
        }
    }
}

mod stack_collection_test {
    use std::collections::HashMap;

    use super::{Stack, StackCollection};

    #[test]
    fn move_between_stacks() {
        let mut sc = get_stack_collection();
        sc.move_between_stacks(3, 1, 2);
        assert_eq!(sc.get_stack(1).len(), 0);
        assert_eq!(sc.get_stack(2).len(), 6);

        sc.move_between_stacks(5, 2, 1);
        assert_eq!(sc.get_stack(1).len(), 5);
        assert_eq!(sc.get_stack(2).len(), 1);

        sc.move_between_stacks(5, 1, 3);
        assert_eq!(sc.get_stack(1).len(), 0);
        assert_eq!(sc.get_stack(3).len(), 8);
    }

    #[test]
    fn test_order() {
        let mut sc = get_stack_collection();
        sc.move_between_stacks(2, 1, 2);
        assert_eq!(sc.get_stack(1).get_top(), 'A');
        assert_eq!(sc.get_stack(2).get_top(), 'B');
    }

    fn get_stack_collection() -> StackCollection {
        let s1 = Stack::from("ABC");
        let s2 = Stack::from("DEF");
        let s3 = Stack::from("GHI");
        let mut stacks = HashMap::new();
        stacks.insert(1, s1);
        stacks.insert(2, s2);
        stacks.insert(3, s3);
        StackCollection { stacks }
    }
}
