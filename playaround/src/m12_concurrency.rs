#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::sync::{Arc, Mutex};
    use std::thread::{spawn, JoinHandle};

    #[test]
    fn test_concurrency() {
        let file_mutex: Arc<Mutex<Result<std::fs::File, std::io::Error>>> = Arc::new(Mutex::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("increments.txt"),
        ));

        // Spawn multiple threads to write to the file concurrently
        // Each thread will increment a number and write it to the file
        // Using Arc to share ownership of the file across threads
        // Using Mutex to ensure that only one thread can write to the file at a time

        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for i in 0..10 {
            let file_mutex_clone = Arc::clone(&file_mutex);
            let handle = spawn(move || {
                let mut file = file_mutex_clone.lock().unwrap();
                match &mut *file {
                    Ok(f) => {
                        writeln!(f, "Increment: {}", i).expect("Failed to write to file");
                    }
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                    }
                }
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
    }
}
