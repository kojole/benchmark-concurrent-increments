use std::thread;

use crossbeam_channel::bounded;

pub fn run(n_threads: usize, count: usize) -> usize {
    let (tx, rx) = bounded(0);
    let mut threads = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let tx = tx.clone();
        let t = thread::spawn(move || {
            for _ in 0..count {
                tx.send(1);
            }
        });
        threads.push(t);
    }
    drop(tx);

    let mut value = 0usize;
    for i in rx {
        value += i;
    }

    for t in threads {
        t.join().unwrap();
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_single() {
        assert_eq!(run(1, 1_000), 1_000);
    }

    #[test]
    fn run_concurrent() {
        assert_eq!(run(4, 250), 1_000);
    }
}
