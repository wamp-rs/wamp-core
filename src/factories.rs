use std::sync::RwLock;

use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBER: RwLock<u64> = RwLock::new(0);
    static ref TOPICS: RwLock<Vec<String>> = RwLock::new(vec![]);
}

/// # Auto incrementer
/// Thread safe Auto Incrementing method that adds 1, and returns the number.
/// 
/// Here is the source code for that particular snippet, as its usage is obvious
/// and this space is used so people can audit it for its "thread safety".
/// ```
/// use lazy_static::lazy_static;
/// use std::sync::RwLock;
/// 
/// lazy_static! {
///     static ref NUMBER: RwLock<u64> = RwLock::new(0);
/// }
/// 
/// pub fn increment() -> u64 {
///     let previous = *NUMBER.read().unwrap();
///     let mut num = NUMBER.write().unwrap();
///     *num = previous + 1;
///     *num
/// }
/// 
/// for i in 1..10 {
///     assert_eq!(i, increment());
/// }
/// ```
pub fn increment() -> u64 {
    let previous = *NUMBER.read().unwrap();
    let mut num = NUMBER.write().unwrap();
    *num = previous + 1;
    *num
}

pub fn add_associated_subscription() {
    
}

pub fn subscribe<T: ToString>(topic: T) {
    let mut current = TOPICS.write().unwrap();
    current.push(topic.to_string())
}

pub fn unsubscribe<T: ToString>(topic: &T) {
    let mut current = TOPICS.write().unwrap();
    current.retain(|i| i != &topic.to_string())
}

pub fn subscription_contains<T: ToString>(topic: &T) -> bool {
    TOPICS.read().unwrap().contains(&topic.to_string())
}