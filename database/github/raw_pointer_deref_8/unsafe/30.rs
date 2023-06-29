#![allow(unused)]

fn main() {
    if let Some(NotificationType::OneWaiter) = unsafe { (*waiter.get()).notified } {
        if let Some(waker) = notify_locked(&mut waiters, &notify.state, notify_state) {
            drop(waiters);
            waker.wake();
        }
    }


}