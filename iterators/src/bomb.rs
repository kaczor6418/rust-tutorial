pub struct Bomb {
    timer: i32,
    limit: i32,
}

impl Bomb {
    pub fn new(limit: i32) -> Bomb {
        return Bomb { timer: 0, limit };
    }
}

impl Iterator for Bomb {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.timer < self.limit {
            self.timer += 1;
            Some(self.timer)
        } else {
            println!("!!!!!!Explosion!!!!!!");
            None
        }
    }
}

#[test]
fn should_explode_after_fifth_took() {
    let mut bomb = Bomb::new(5);
    assert!(bomb.next().is_some());
    assert!(bomb.next().is_some());
    assert!(bomb.next().is_some());
    assert!(bomb.next().is_some());
    assert!(bomb.next().is_some());
    assert!(bomb.next().is_none());
}
