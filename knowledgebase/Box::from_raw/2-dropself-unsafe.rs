/*
    Replaceable: No
*/

/*
struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }
}

struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node::new(value)));
        if self.head.is_null() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            unsafe {
                (*new_node).prev = self.tail;
                (*self.tail).next = new_node;
                self.tail = new_node;
            }
        }
    }
}
*/
impl<T> Drop for DoublyLinkedList<T> {
*/
    fn drop(&mut self) {
        let mut current_node = self.head;
        while !current_node.is_null() {
            unsafe {
                let next_node = (*current_node).next;
                Box::from_raw(current_node);
                current_node = next_node;
            }
        }
    }
/*
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
}
*/
