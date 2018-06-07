use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 新しいThreadPoolを生成する。
    /// sizeがプールのスレッド数です。
    /// 
    /// #Panics
    /// 
    /// sizeが0なら`new`関数はパニックする。
    // pub fn new(size: u32) -> Result<ThreadPool, PoolCreationError> { 
    pub fn new(size: usize) -> ThreadPool { 
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);


        for _ in 0..size {

        }

        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        
    }

}