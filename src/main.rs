use std::thread;

fn main() {
    let current_thread = thread::current();
    println!("{} : Hello, world!", current_thread.name().unwrap());
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 0..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application done");
        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }
            return counter;
        });

        println!("Waiting Handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Total Counter : {}", counter),
            Err(error) => println!("Error : {:?}", error)
        }

        println!("Application done");
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 0..=5 {
            match current.name() {
                None => { println!("{:?} : Counter : {}", current.id(), i); }
                Some(name) => { println!("{} : Counter : {}", name, i); }
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();
        println!("Total Counter 1 : {}", result1);
        println!("Total Counter 2 : {}", result2);
        println!("Application Finish");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => { println!("Total Counter 1 : {}", counter) }
            Err(error) => { println!("Error : {:?}", error) }
        }

        match result2 {
            Ok(counter) => { println!("Total Counter 2 : {}", counter) }
            Err(error) => { println!("Error : {:?}", error) }
        }

        println!("Application Finish");
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        println!("{}", current_thread.name().unwrap());

        let name = String::from("Eko");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello {}", name);
        };

        let handle = thread::spawn(closure);
        handle.join().unwrap();
    }
}