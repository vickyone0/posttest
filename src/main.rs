use reqwest::blocking::Client;
use std::collections::HashMap;
use std::thread;


fn main() {
    let mut v = Vec::<std::thread::JoinHandle<()>>::new();
    for i in 0..100{
    let handle =  thread::spawn(move || {
        let mut map = HashMap::new();
         map.insert("grant_type", "password");
    let client = Client::new();

    let res = client.post("https://httpbin.org/post")
        .json(&map)
        .send();
           

});
    v.push(handle);

    
  }
  for handle in v {
        handle.join().unwrap();
    }
}

