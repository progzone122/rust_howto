use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

// Our error type needs to be `Send` to be used in a channel
fn read_contents<T: AsRef<Path>>(
    file: T,
) -> Result<String, Box<dyn Error + Send>> {
    Ok(file.as_ref().to_string_lossy().into_owned())
}

fn main() {
    let (tx, rx) = mpsc::channel(); // to share state between threads, consider using a channel

    thread::scope(|scope| {
        // Creates a “fork-join” scope
        let tx2 = tx.clone();
        scope.spawn(move || {
            println!("hello from the first scoped thread");
            let contents = read_contents("foo.txt");
            tx.send(contents).unwrap();
        });
        scope.spawn(move || {
            println!("hello from the second scoped thread");
            let contents = read_contents("bar.txt");
            tx2.send(contents).unwrap();
        });
    });
    // No join; spawned threads get joined automatically once the scope ends!

    // Receive messages from the channel
    println!("hello from the main thread");

    for received in rx {
        println!("Got: {:?}", received);
    }
}
