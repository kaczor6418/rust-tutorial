pub struct Hotel {
    pub rooms: i32,
    pub guests: i32
}

impl Hotel {
    pub fn new(rooms: i32, guests: i32) -> Hotel {
        return Hotel {
            rooms,
            guests
        };
    }

    pub fn add_guest(&mut self) {
        if self.can_add_guest() {
            self.guests += 1;
        } else {
            println!("Hotel is full");
        }
    }

    pub fn remove_guest(&mut self) {
        if self.can_remove_guest() {
            self.guests -= 1;
        } else {
            println!("There is no more guests in hotel_manager");
        }
    }

    fn can_add_guest(&self) -> bool {
        return self.free_rooms() > 0;
    }

    fn can_remove_guest(&self) -> bool {
        return self.guests > 0;
    }

    fn free_rooms(&self) -> i32 {
        return self.rooms - self.guests;
    }

}
