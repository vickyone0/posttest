use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..100 {
        let handle = thread::spawn(move || {
            println!("Hello from thread {}", i);
            let client = reqwest::blocking::Client::new();
            let res = client
                .post("http://httpbin.org/post")
                .body("the exact body that is sent")
                .send();
            println!("Response from thread {}: {:?}", i, res);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
