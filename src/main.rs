#[derive(Debug, Default, Clone)]
pub struct What<'a> {
    name: &'a str,
    children: Vec<What<'a>>,
}

impl<'a> What<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            children: vec![],
        }
    }

    pub fn add(&mut self, child: What<'a>) {
        self.children.push(child);
    }

    pub fn iter<'i>(&'a self) -> WhatIter<'i>
    where
        'a: 'i,
    {
        WhatIter::<'i>::new(self)
    }

    pub fn recursive_iter<'i>(&'a self) -> WhatRecursiveIter<'i>
    where
        'a: 'i,
    {
        WhatRecursiveIter::<'i>::new(self)
    }

    pub fn has_children(&self) -> bool {
        self.children.len() > 0
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

#[derive(Clone)]
pub struct WhatIter<'i> {
    inner_iter: std::slice::Iter<'i, What<'i>>,
}

impl<'i> WhatIter<'i> {
    pub fn new(y: &'i What<'i>) -> Self
    where
        Self: 'i,
    {
        Self {
            inner_iter: y.children.iter(),
        }
    }
}

impl<'i> Iterator for WhatIter<'i> {
    type Item = &'i What<'i>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

#[derive(Clone)]
pub struct WhatRecursiveIter<'r> {
    inner_iter_stack: Vec<WhatIter<'r>>,
    inner_junctions: Vec<&'r What<'r>>,
}

impl<'r> WhatRecursiveIter<'r> {
    pub fn new(what: &'r What<'r>) -> Self
    where
        Self: 'r,
    {
        Self {
            inner_iter_stack: vec![what.iter()],
            inner_junctions: vec![what],
        }
    }
}

impl<'i> Iterator for WhatRecursiveIter<'i> {
    type Item = &'i What<'i>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // get the top_of_stack
            if let Some(top_of_stack) = self.inner_iter_stack.last_mut() {
                // get the top of stack iterators first item
                if let Some(top_of_stack_child) = top_of_stack.next() {
                    // it has children, and now we want to go depth first,
                    // so check if that child has children too
                    // note that the iterator will always be valid
                    if top_of_stack_child.has_children() {
                        // it does, so now we push it to the stack
                        // and do another loop iteration
                        self.inner_iter_stack.push(top_of_stack_child.iter());
                        self.inner_junctions.push(top_of_stack_child);
                        continue;
                    } else {
                        // it doesn't so, it's a leaf -> instant return
                        return Some(top_of_stack_child);
                    }
                }

                if let Some(nxt) = top_of_stack.next() {
                    return Some(nxt);
                } else {
                    let _ = self.inner_iter_stack.pop();
                    return self.inner_junctions.pop();
                }
            } else {
                break;
            }
        }
        None
    }
}

fn main() {
    println!("cargo test please");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mut root = What::new("root");
        let mut a0 = What::new("a0");
        let mut a1 = What::new("a1");
        let a0b0 = What::new("a0b0");
        let a0b1 = What::new("a0b1");
        let a1b0 = What::new("a1b0");
        let mut a1b1 = What::new("a1b1");
        let mut a1b1c0 = What::new("a1b1c0");
        let a1b1c0d0 = What::new("a1b1c0d0");

        // reverse tree buildup
        a1b1c0.add(a1b1c0d0);
        a1b1.add(a1b1c0);
        a1.add(a1b0);
        a1.add(a1b1);

        a0.add(a0b0);
        a0.add(a0b1);

        root.add(a0);
        root.add(a1);

        root.iter()
            .for_each(|x| println!("Iter(plain): {}", x.name()));
        root.recursive_iter()
            .for_each(|x| println!("Iter(recursive): {}", x.name()));
    }
}
