fn main() {
    let my_rec = Quadrilateral {
        width: 10,
        height: 20,
    };
    println!("{}", my_rec.get_area());
    let half_rec = my_rec.make_half();
    println!("Width: {} Height: {}", half_rec.width, half_rec.height);
}

struct Quadrilateral {
    width: u32,
    height: u32,
}

impl Quadrilateral {
    fn get_area(&self) -> u32 {
        self.height * self.width
    }
    fn make_half(&self) -> Quadrilateral {
        Quadrilateral {
            width: self.width / 2,
            height: self.height / 2,
        }
    }
}
