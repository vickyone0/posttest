use std::thread;

fn main() {

    let mut handles = vec![];

    for i in 0..1000 {

        let handle = thread::spawn(move || {
            let res = reqwest::blocking::Client::new()
            .get("https://www.gecidukki.ac.in/")
            .body("Hello, world!")
            .send();
            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Thread {}: Request successful", i);
                    } else {
                        println!("Thread {}: Request failed with status: {}", i, response.status());
                    }
                },
                Err(e) => println!("Thread {}: Request error: {:?}", i, e),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => println!("Thread completed successfully"),
            Err(e) => println!("Thread failed: {:?}", e),
        }
    }
}