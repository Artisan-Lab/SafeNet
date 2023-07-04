unsafe fn sum_list(head: *const Node) -> i32 {
    let mut sum = 0;
    let mut curr = head;
    while !curr.is_null() {
        let val = (*curr).val;
        sum += val;
        curr = (*curr).next;
    }
    sum
}
