#[derive(Debug)]
pub struct Queue<T> {
    array: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue::<T> { array: vec![] }
    }

    pub fn from(elements: Vec<T>) -> Queue<T> {
        Queue { array: elements }
    }

    pub fn append(&mut self, element: T) {
        let mut new_array: Vec<T> = vec![element];
        new_array.append(&mut self.array);
        self.array = new_array;
    }

    pub fn next(&mut self) -> Option<T> {
        let next = self.array.pop();
        next
    }

    pub fn is_empty(&self) -> bool {
        if self.array.len() == 0 {
            true
        } else {
            false
        }
    }
}
