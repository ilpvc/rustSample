#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::*;
    #[test]
    fn test1(){
        /**
        * 由于 Rust 的测试框架（如 cargo test）会自动启动和管理测试线程，
        * 它们会等待所有线程执行完成，以确保测试的正确性。因此，在测试中，所有
        * 的线程会被按顺序执行，而不是并发执行，这样可以更容易地观察和理解输出，
        * 同时也确保测试的可重复性。
        */

        let mut handles = vec![];
        for i in 1..9 {
            let handle = thread::spawn(move || {
                for j in 1..9 {
                    println!("{}线程中的：{}", i, j)
                }
            });
            handles.push(handle)
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}