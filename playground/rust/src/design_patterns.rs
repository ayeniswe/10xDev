use std::{
    any::{self, Any},
    collections::{HashMap, HashSet},
};

pub fn decorator() {
    // Decorator Pattern

    /// Old Logger class
    pub struct Logger();
    impl LoggerWrapper for Logger {
        fn log(&self) {
            println!("Message logged to file")
        }
    }

    /// Base wrapper for old `Logger`
    pub trait LoggerWrapper {
        fn log(&self);
    }

    /// Add error logging capabilites to `Logger`
    pub struct ErrorLogger<Wrapper: LoggerWrapper>(pub Wrapper);
    impl<Wrapper: LoggerWrapper> LoggerWrapper for ErrorLogger<Wrapper> {
        fn log(&self) {
            self.0.log();
            println!("Error Message logged to file")
        }
    }

    /// Add warn logging capabilites to `Logger`
    pub struct WarnLogger<Wrapper: LoggerWrapper>(pub Wrapper);
    impl<Wrapper: LoggerWrapper> LoggerWrapper for WarnLogger<Wrapper> {
        fn log(&self) {
            self.0.log();
            println!("Warn Message logged to file")
        }
    }

    let stack = Logger();
    let stack = ErrorLogger(stack);
    let stack = WarnLogger(stack);
    stack.log()
}

pub fn publisher() {
    pub enum Subject<'a> {
        Fruit(&'a str),
        Veggie(&'a str),
        Meat(&'a str),
    }

    pub trait Publisher<T: Consumer> {
        fn add_consumer(&mut self, consumer: T);
        fn remove_consumer(&mut self, consumer: &T);
        fn notify(&self);
    }

    pub trait Consumer {
        fn update(&self, sub: &str);
        fn add_subscription(&mut self, sub: Subject);
        fn remove_subscription(&mut self, sub: Subject);
        fn get_subscription(&self) -> Vec<Subject>;
    }

    pub struct MainPublisher<T: Consumer + Eq + std::hash::Hash> {
        consumers: HashSet<T>,
    }
    impl<T: Consumer + Eq + std::hash::Hash> Publisher<T> for MainPublisher<T> {
        fn add_consumer(&mut self, consumer: T) {
            self.consumers.insert(consumer);
        }
        fn remove_consumer(&mut self, consumer: &T) {
            self.consumers.remove(consumer);
        }
        fn notify(&self) {
            for consumer in self.consumers.iter() {
                for subscription in consumer.get_subscription() {
                    match subscription {
                        Subject::Fruit(_) => consumer.update("Fruits: Here is the orange you asked for"),
                        Subject::Meat(_) => consumer.update("Veggie: Here is the broccoli you asked for"),
                        Subject::Veggie(_) => consumer.update("Meat: Here is the fish you asked for"),
                    }
                }
            }
        }
    }
}
