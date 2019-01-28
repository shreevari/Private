use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;
//use std::rc::Rc;

fn main() {
	//message_passing();
	shared_memory();
}

fn shared_memory() {
	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();

			*num +=1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	println!("Result: {}", *counter.lock().unwrap());
}

fn message_passing() {
	let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);

	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
		
	});

	thread::spawn(move || {
		let vals = vec![
			String::from("more"),
			String::from("mesages"),
			String::from("for"),
			String::from("you"),
		];

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
		
	});	

	for recieved in rx {
		println!("Got {}", recieved);		
	}
}