use std::boxed::Box; 

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    item: T,
    next: Link<T>
}

pub struct LinkedList<T> {
    head: Link<T>
}

pub trait Drop {
    fn drop(&mut self);
}

pub trait Iter {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct IntoIterator<T> {
    iterator: LinkedList<T>
}

pub struct Iterator<'a, T> {
    next: Option<&'a Node<T>>
}

pub struct MutIterator<'a, T> {
    next: Option<&'a mut Node<T>>
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {head: None}
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item: item,
            next: self.head.take() 
        });
        self.head = Some(new_node);
    }    

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.item
        })
    }

    pub fn mut_peek(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.item
        })
    }

    pub fn into_iter(self) -> IntoIterator<T> {
        IntoIterator { iterator: self }
    }
    
    pub fn iter<'a> (&'a self) -> Iterator<'a, T> {
        Iterator { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn mut_iter<'a> (&'a mut self) -> MutIterator<'a, T> {
        MutIterator {next: self.head.as_deref_mut()}
    }
}


impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut node_in_box) = curr {
            curr = node_in_box.next.take()
        }
    }
}


impl<T> Iter for IntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.pop()
    }
}


impl<'a, T> Iter for Iterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.item
        })
    }
}


impl<'a, T> Iter for MutIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.item
        })
    }
}


#[macro_export]
macro_rules! linked_list {
   
    () => { LinkedList::new() };

    ($ ( $x:expr ),* ) => {
        {
            let mut temp_list = LinkedList::new();
            $( temp_list.push($x); )*
            temp_list    
        }
    };
}


#[cfg(test)] 
mod test {
    
    use super::LinkedList;
    use super::Iter;
    
    #[test]
    fn base_case() {
        let mut llist: LinkedList<i32> = LinkedList::new();

        llist.push(1);
        llist.push(2);
        llist.push(3);


        assert_eq!(llist.pop(), Some(3));
        assert_eq!(llist.pop(), Some(2));
    }

    #[test]
    fn peek_and_mutable() {
        let mut llist: LinkedList<i32> = LinkedList::new();

        assert_eq!(llist.peek(), None);
        assert_eq!(llist.mut_peek(), None);
        
        llist.push(1);
        assert_eq!(llist.peek(), Some(&1));
        assert_eq!(llist.mut_peek(), Some(&mut 1));

        llist.push(2);
        assert_eq!(llist.peek(), Some(&2));
        assert_eq!(llist.mut_peek(), Some(&mut 2));

        llist.mut_peek().map(|value| {
            *value = 71 
        });

       assert_eq!(llist.peek(), Some(&71));
       assert_eq!(llist.pop(), Some(71));
       
    }

    #[test]
    fn into_iter() {
        let mut llist: LinkedList<i32> = LinkedList::new();
        llist.push(1); llist.push(2); llist.push(3);
        
        let mut iter_list = llist.into_iter();
        assert_eq!(iter_list.next(), Some(3));
        assert_eq!(iter_list.next(), Some(2));
        assert_eq!(iter_list.next(), Some(1));
        assert_eq!(iter_list.next(), None);
    }

    #[test]
    fn iter() {
        let mut llist: LinkedList<i32> = LinkedList::new();
        llist.push(1); llist.push(2); llist.push(3);

        let mut iter_list = llist.iter();
        assert_eq!(iter_list.next(), Some(&3));
        assert_eq!(iter_list.next(), Some(&2));
        assert_eq!(iter_list.next(), Some(&1));
        assert_eq!(iter_list.next(), None);

    }

    #[test]
    fn mut_iter() {
        let mut llist: LinkedList<i32> = LinkedList::new();
         llist.push(1); llist.push(2); llist.push(3);

        let mut iter_list = llist.mut_iter();
        assert_eq!(iter_list.next(), Some(&mut 3));
        assert_eq!(iter_list.next(), Some(&mut 2));
        assert_eq!(iter_list.next(), Some(&mut 1));
    }

    #[test]
    fn list_macro() {
        let empty_list: LinkedList<i32> = linked_list![];
        let int_list: LinkedList<i32> = linked_list![1, 2, 3];
        let char_list: LinkedList<char> = linked_list!['a', 'b', 'c'];
        
        assert_eq!(empty_list.peek(), None);
        assert_eq!(int_list.peek(), Some(&3));
        assert_eq!(char_list.peek(), Some(&'c'));

    }
}

