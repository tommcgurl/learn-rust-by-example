#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {x: tl_x, y: tl_y},
        bottom_right: Point {x: br_x, y: br_y}
    } = rect;
    let width = br_x - tl_x;
    let height = tl_y - br_y;
    width * height
}

fn square(point: Point, height_width: f32) -> Rectangle {
    let Point { x: bl_x, y: bl_y } = point;

    let top_left = Point {
        x: bl_x,
        y: height_width
    };
    let bottom_right = Point {
        x: height_width,
        y: bl_y
    };
    Rectangle {
        top_left,
        bottom_right
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Call the rect_area function.
    let new_rect = Rectangle {
        top_left: Point { x: 0.0 , y: 100.0},
        bottom_right: Point { x: 10.0 , y: 0.0}
    };
    let new_rect_area = rect_area(&new_rect);
    println!("The area for the rectangle is {}", new_rect_area);

    // Call the square funtion.
    let bottom_left = Point { x: 0.0, y: 0.0 } ;
    let square_rect = square(bottom_left, 10.0);

    let square_rect_area = rect_area(&square_rect);
    println!("The area for the square is {}", square_rect_area);


}
