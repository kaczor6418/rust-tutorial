mod hotel_manager;

use hotel_manager::{hotel::Hotel, guest::Guest};

fn main() {
    let mut hotel = Hotel::new(2, 0);
    hotel.add_guest();
    hotel.add_guest();
    hotel.add_guest();
    println!("Guests in hotel: {}", hotel.guests);
    hotel.remove_guest();
    hotel.remove_guest();
    hotel.remove_guest();
    println!("Guests in hotel: {}", hotel.guests);
}
