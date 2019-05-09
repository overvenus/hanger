use hanger::hook;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn test_hooks() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    fn add_one() {
        COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    fn sub_one() {
        COUNTER.fetch_sub(1, Ordering::Relaxed);
    }

    #[hook(add_one)]
    fn entry_add_one() {
        COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    entry_add_one();
    assert_eq!(COUNTER.load(Ordering::Relaxed), 2);

    #[hook(before = add_one, after = sub_one)]
    fn entry_add_sub() {
        COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    entry_add_sub();
    assert_eq!(COUNTER.load(Ordering::Relaxed), 3);
}

#[test]
fn test_path_hooks() {
    mod path {
        use super::*;
        pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

        pub fn add_one() {
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
        pub fn sub_one() {
            COUNTER.fetch_sub(1, Ordering::Relaxed);
        }
    }

    #[hook(path::add_one)]
    fn entry_add_one() {
        path::COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    entry_add_one();
    assert_eq!(path::COUNTER.load(Ordering::Relaxed), 2);

    #[hook(before = path::add_one, after = path::sub_one)]
    fn entry_add_sub() {
        path::COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    entry_add_sub();
    assert_eq!(path::COUNTER.load(Ordering::Relaxed), 3);
}

mod tests {
    use super::*;
    static G_COUNTER: AtomicUsize = AtomicUsize::new(0);
    fn add_g() {
        G_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    #[test]
    #[hook(add_g)]
    fn test_hook_tests() {
        assert_eq!(G_COUNTER.load(Ordering::Relaxed), 1);
    }
}
