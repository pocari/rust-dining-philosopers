use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosoper {
    name: String,
    left: usize,
    right: usize,
}

impl Philosoper {
    fn new(name: &str, left: usize, right: usize) -> Philosoper {
        Philosoper {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let phirosopers = vec![Philosoper::new("Judith Butler", 1, 0),
                           Philosoper::new("Gilles Deleuze", 2, 1),
                           Philosoper::new("Karl Marx", 3, 2),
                           Philosoper::new("Emma Goldman", 4, 3),
                           Philosoper::new("Michel Foucault", 4, 0)];

    let table = Arc::new(Table {
        forks: vec![Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(())],
    });

    let handles: Vec<_> = phirosopers.into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
