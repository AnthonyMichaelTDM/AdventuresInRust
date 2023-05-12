/// Type alias for the link between 2 nodes
type Link<T> = Option<Box<Node<T>>>;

/// Representation of an element in the linked list
pub struct Node<T>
where
    T: Default,
{
    elem: T,
    next: Link<T>,
}

impl<T> Default for Node<T>
where
    T: std::default::Default,
{
    /// Constructor
    fn default() -> Self {
        Self {
            elem: Default::default(),
            next: None,
        }
    }
}

pub struct List<T>
where
    T: std::default::Default,
{
    head: Link<T>,
    #[allow(dead_code)]
    size: usize,
}

impl<T> Default for List<T>
where
    T: std::default::Default,
{
    /// Constructor
    fn default() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
}

impl<T> Drop for List<T>
where
    T: std::default::Default,
{
    /// Destructor
    fn drop(&mut self) {
        // traverse stack, taking ownership of nodes as you go
        // taking ownership will cause an implicit drop every loop iteration
        let mut curr = self.head.take();
        
        while let Some( mut boxed_node ) = curr {
            curr = boxed_node.next.take();
        }
    }
}

impl<T> List<T>
where
    T: std::default::Default,
{
    #[allow(dead_code)]
    /// inserts elem to the top of the queue
    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));

        self.size += 1;
    }

    /// removes the last inserted element
    pub fn pop(&mut self) -> Option<T> {
        self.head
            .take() // take self.head, without borrowing or taking ownership
            .map(|node| {
                self.head = node.next; // move head to the "next" node
                self.size -= 1;
                node.elem // return the value of the node
                          // implicitly drop the old head
            })
    }

    #[allow(dead_code)]
    /// tops the top element of the stack
    pub fn top(&self) -> Option<&T> {
        // convert a reference to an option containing a node into
        // a option containing a reference to a node,
        // then return a reference to the elem of the node contained in the option
        self.head.as_ref().map(|node| &node.elem)
    }

    #[allow(dead_code)]
    /// tops the top element of the stack
    pub fn top_mut(&mut self) -> Option<&mut T> {
        // same as top(), but mutable
        self.head.as_mut().map(|node| &mut node.elem)
    }

    #[allow(dead_code)]
    /// number of elements in the stack
    pub fn size(&self) -> usize {
        self.size
    }

    #[allow(dead_code)]
    /// a boolean value indicating whether no elements are stored
    pub fn empty(&self) -> bool {
        self.size == 0
    }

    #[allow(dead_code)]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    #[allow(dead_code)]
    /// Returns the iter of this [`List<T>`].
    fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    #[allow(dead_code)]
    /// Returns the iter of this [`List<T>`].
    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(List<T>)
where
    T: std::default::Default;

impl<T> Iterator for IntoIter<T>
where
    T: std::default::Default,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T>
where
    T: std::default::Default,
{
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: std::default::Default,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // move the pointer
            self.next = node.next.as_deref(); // we use as_deref to peel open the box
                                              // return reference to element at pointer
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: std::default::Default> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: std::default::Default,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // we don't want to borrow the content, so lets just steal it with take()
            // move the pointer
            self.next = node.next.as_deref_mut(); // we use as_deref to peel open the box
                                                  // return reference to element at pointer
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::default();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.size(), 3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.size(), 1);

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);
        assert_eq!(list.size(), 3);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.size(), 1);

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.size(), 0);
        assert!(list.empty());

    }

    #[test]
    fn top() {
        let mut list = List::default();
        assert_eq!(list.top(), None);
        assert_eq!(list.top_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.top(), Some(&3));
        assert_eq!(list.top_mut(), Some(&mut 3));

        if let Some(value) = list.top_mut() {
            *value = 42
        };

        assert_eq!(list.top(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::default();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::default();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::default();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
