// structs can have reference fields
// but lifetime annotations are needed
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs
// these do not get coerced in function params
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct PointStruct {
    x: i32,
    y: i32,
}

// unit-like structs
// no fields
// used as markers
struct UnitLikeStruct;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for self : &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // self is moved unless Copy trait is implemented
    // even when passed in as a mutable reference
    // moving out of self is safe when there is no heap data
    // and rectangle struct can add #[derive(Copy, Clone)] for this
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // all fields are mutable
    let mut user1 = User {
        email: String::from("address@email.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("different@email.com");

    let user2 = User {
        active: user1.active,
        sign_in_count: user1.sign_in_count,
        email: String::from("secondaddress@email.com"),
        username: String::from("secondusername123"),
    };

    // spread operator is like js
    // spread must be last
    // user1 can't be used after this
    // since username (string in heap, no copy trait) was moved
    let user3 = User {
        email: String::from("thirdaddress@email.com"),
        ..user1
    };

    println!("user3 email: {}", user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // struct and tuple without fields
    let unit_like_struct = UnitLikeStruct;
    let unit_tuple = ();

    let mut point = PointStruct { x: 0, y: 0 };
    let x = &mut point.x; // p and p.x can not be used
                          // until x goes out of scope
                          // point.x += 1;
    println!("point.x: {}", point.x);
    println!("point.y: {}", point.y);

    print_point(&point);
    // *x += 1; // error: cannot borrow as immutable because it is
    // also borrowed as mutable

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect area: {}", rect.area());

    let sq = Rectangle::square(3);
    println!("sq area: {}", sq.area());

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // rect2.set_width(40); // cannot borrow immutable as mutable
    // let imm_rect_ref = &rect2;
    // imm_rect_ref.set_width(40);

    let mut rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    rect3.set_width(40);

    // let imm_rect_ref = &rect3;  // cannot borrow immutable as mutable
    // imm_rect_ref.set_width(50);
}

// accessing borrowed struct fields does not incur a move
// borrowing structs happens often
fn rect_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn print_point(p: &PointStruct) {
    println!("{}, {}", p.x, p.y);
}

fn build_user(email: String, username: String) -> User {
    // field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
