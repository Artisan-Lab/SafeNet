use std::mem;
use std::pin::Pin;

fn move_pinned_ref<T>(mut a: T, mut b: T) {
    let mut p_a = Pin::new(&mut a);
    // Pin `a`
    mem::swap(&mut p_a, &mut Pin::new(&mut b));
    // Swap the pins instead of the values
    mem::swap(&mut a, &mut b);
    // Swap the values
    // Now `a` is still pinned and cannot be moved
}
