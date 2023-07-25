// structs can have reference fields
// but lifetime annotations are needed
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs
// used for naming purposes
// these do not get coerced in function params

struct Color(i32, i32, i32);
// let black = Color(0, 0, 0);
struct Point(i32, i32, i32);

struct PointStruct {
    x: i32,
    y: i32,
}

// unit-like structs
// no fields
// used to define types with traits but no data
struct UnitLikeStruct;
// let unit_like_struct = UnitLikeStruct;

// Attributes are metadata about pieces of Rust code
// attribute to automatically implement the Debug trait
// allows for printing of a struct or enum
// in a debug format using the {:?} format specifier in println!
// or {:#?} for pretty print

// note: dbg! macro can also print
// it takes ownership of the value passed in
// and prints the line number and file where dbg! was called

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // functions inside an impl block are called methods (have self param)
    // or associated functions (don't have self param)

    // &self is short for self : &Self
    // self can be &mut self or self also
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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // called like this
    // Rectangle::square(3)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // all fields are mutable
    // you can not mark a subset of fields as mutable
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

    // spread operator is like js (2 dots though)
    // spread must be last
    // user1 can't be used after this
    // since username (string in heap, no copy trait) was moved
    // if username was a primitive, it would be copied
    // and user1 could still be used
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
                          // and like with tuples, the compiler considers all fields to be borrowed
                          // when one field is borrowed in a function
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

    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    let max_rect = rect4.max(rect5);
    println!("max rect: {:?}", max_rect);
    // println!("rect 4 area: {}", rect4.area()); // error: rect4 was moved
    // the same kind of error would happen with self references in methods
    // in this case, the rect does not have heap data (e.g. String), so it can implement Copy trait
    // with #[derive(Copy, Clone)] and then it would be copied instead of moved

    // rust automatically inserts *s and &s as needed for method calls
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
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
    // instead of `username: username`, for example
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
