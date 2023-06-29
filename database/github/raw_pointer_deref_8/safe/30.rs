#![allow(unused)]

fn main() {
    if let NotificationState::OneWaiter = w.notified {
        if let Some(waker) = notify_locked(&mut lists.waiters, &notify.state, notify_state)
        {
            drop(lists);
            waker.wake();
        }
    }


}