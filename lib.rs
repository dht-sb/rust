pub mod LinkedList {
    #[derive(Debug)]
    pub struct ListNode<T> {
        value: T,
        next: Option<Box<ListNode<T>>>,
    }
    impl<T> ListNode<T> {
        fn to_self(&mut self) -> &mut Self {
            let mut b = self;
            if let Some(ref mut a) = b.next {
                b = a;
            }
            b
        }
        fn to_to_self(&mut self) -> &mut Self {
            let mut a = self;
            while let Some(ref mut b) = a.next {
                a = b;
            }
            a
        }
        fn new(value: T) -> Self {
            return ListNode { value, next: None };
        }
        fn to_bool(&self) -> bool {
            let mut v = false;
            if let Some(_) = self.next {
                v = true;
            }
            v
        }
    }
    #[derive(Debug)]
    pub struct LinkedList<T> {
        head: Option<Box<ListNode<T>>>,
    }
    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList { head: None }
        }
        pub fn push(&mut self, value: T) {
            let mut value = ListNode::new(value);
            match self.head {
                Some(ref mut a) => {
                    let mut b = a.to_to_self();
                    b.next = Some(Box::new(value));
                }
                None => {
                    self.head = Some(Box::new(value));
                }
            }
        }
        pub fn pop(&mut self, value: &mut T) -> bool {
            let mut nf = false;
            if let Some(ref mut a) = self.head {
                if a.to_bool() {
                    let mut d = &mut **a as *mut ListNode<T>;
                    let mut dd;
                    unsafe {
                        dd = &mut *d;
                    }
                    let mut b = a.to_self();
                    std::mem::swap(b, dd);
                    let mut b = &mut b.value;
                    std::mem::swap(b, value);
                    nf = true;
                }
            }
            nf
        }
    }
}
