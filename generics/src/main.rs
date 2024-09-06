// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl <T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }


// struct PointTwo<T, U> {
//     x: T,
//     y: U,
// }



// fn main() {
//     let number_list = vec![1, 23, 32, 232];

//     let result = largest(&number_list);

//     let char_list = vec!['a', 's', 'f', 'a'];
    
//     let result = largest(&char_list);

//     let integer = Point { x: 1, y: 1};
//     let float =  Point { x : 1.1, y: 1.1 };

//     print!("integer.x = {}", integer.x());

//     let combo = PointTwo { x: 1, y: 1.1 };
// }



///////////////////
//  EXERCISE 2   //
//////////////////

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point<X1, Y1>{ 
    fn mixup<X2, Y2> (self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: 4, y: 11.3 };

    let p3 = p1.mixup(p2);

    print!("p3.x {}, p3.y {}", p3.x, p3.y);
}
