#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn channel() {
        let (sender1, receiver) = mpsc::channel::<u8>();

        let sender2 = sender1.clone();

        thread::spawn(move || {
            sender1.send(1).unwrap();
        });
        thread::spawn(move || {
            sender2.send(2).unwrap();
        });

        let m1 = receiver.recv().unwrap();
        println!("main thread recv: {}", m1);
        assert_eq!(1, m1);

        let m2 = receiver.recv().unwrap();
        println!("main thread recv: {}", m2);
        assert_eq!(2, m2);
    }
}
