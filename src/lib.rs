#![feature(const_fn)]

extern crate core;

pub mod list;

#[cfg(test)]
mod tests {
    use list::ListNode; 
    use list::List;
    use list::ListLink; 
    
    struct Numbernode<'a>{
        number: i32,
        next: ListLink<'a, Numbernode<'a>>
    }

    impl<'a> ListNode<'a, Numbernode<'a>> for Numbernode<'a>{
        fn next(&'a self) -> &'a ListLink<'a, Numbernode<'a>> {
            &self.next
        }
    }
    
    // Note because of this equality being defined as it is, numbers need to be 
    // unique ( otherwise the linked list might be corrupted).
    impl<'a> PartialEq for Numbernode<'a> {
        fn eq(&self, other: &Numbernode<'a>) -> bool {
            self.number == other.number
        }
    }

    fn try_test<'a, 'b, 'c>(a : &'a Numbernode<'a>, b: &'b Numbernode<'a>, 
        c: &'c Numbernode<'a>) where 'b: 'a, 'c: 'a {
        let mut number_list: List<Numbernode<'a>> = List::new();
        
        // Assert the list returns none if empty
        assert!(number_list.pop_tail().is_none());
        
        // Assert the list returns the head if one element is there and none after
        number_list.push_head(a);    
        assert_eq!(number_list.pop_tail().unwrap().number,a.number);
        assert!(number_list.pop_tail().is_none());

        // Assert the list can handle multiple items and output them in the correct
        // order
        number_list.push_head(a);    
        number_list.push_head(b);
        number_list.push_head(c);
        
        assert_eq!(number_list.pop_tail().unwrap().number,a.number);
        assert_eq!(number_list.pop_tail().unwrap().number,b.number);
        assert_eq!(number_list.pop_tail().unwrap().number,c.number);
        assert!(number_list.pop_tail().is_none());
        
    }

    #[test] 
    fn test_list() {
        let (vx, vx2, vx3) =    (Numbernode { number:1, next: ListLink::empty()}, 
                                Numbernode { number:2, next: ListLink::empty()}, 
                                Numbernode { number:3, next: ListLink::empty()});
        
        try_test(&vx, &vx2, &vx3);
    }
}
