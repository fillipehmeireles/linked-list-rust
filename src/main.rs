#[derive(Debug)]
struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T>
where
    T: std::fmt::Display,
{
    fn new() -> Self {
        LinkedList(None)
    }
    fn push_front(&mut self, data: T) {
        let current_first_data = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(current_first_data))));
    }

    fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    fn push_at(&mut self, data: T, index: i16, mut curr_index: i16) {
        if curr_index < index {
            curr_index += 1;
            if let Some((_, ref mut child)) = self.0 {
                child.push_at(data, index, curr_index)
            }
        } else {
            self.push_front(data);
        }
    }
    fn show(&mut self) {
        if let Some((data, _)) = &self.0 {
            print!("{},", data);
            if let Some((_, ref mut child)) = self.0 {
                child.show();
            }
        }
    }
}

fn main() {
    let c_index: i16 = 1;
    let mut linked_list = LinkedList::new();
    linked_list.push_front(3);
    linked_list.push_back(12);
    linked_list.push_front(8);
    linked_list.push_at(5, 2, c_index);
    println!("linked list = {:?}", linked_list);

    linked_list.show()
}
