/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/


#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { size: 0, data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|val| {
            self.size -= 1;
            val
        })
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

fn bracket_match(brackets: &str) -> bool {
    let mut stack = Stack::new();
    for c in brackets.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => {
                let top = stack.pop();
                match (top, c) {
                    (Some('('), ')') | (Some('{'), '}') | (Some('['), ']') => (),
                    _ => return false,
                }
            }
            _ => continue,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
