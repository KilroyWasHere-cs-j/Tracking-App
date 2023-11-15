pub struct Queue {
    pub queue: Vec<String>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, request: String) {
        self.queue.push(request)
    }

    pub fn dequeue(&mut self) -> String {
        self.queue.remove(0)
    }

    pub fn print_queue(&mut self) {
        for i in &self.queue {
            println!("{}", i)
        }
    }
}
