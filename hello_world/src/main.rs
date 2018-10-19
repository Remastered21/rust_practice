fn main() {
    // // let name = "Jackson";
    // let name: &str = "Jackson"; // Specifies the type of the variable

    // let mut age = 42; // this makes the variable mutable
    //                   /* just 'let' is the same as const in JS, 'let mut' is similar to regular var or let in JS*/
    // age += 1;
    // println!("Hello, {}, age {}!", name, age);

    // let number1 = 24;
    // let number2 = 42;
    // if number1 > number2 {
    //     // only boolean conditions are accepted for 'if'.
    //     println!("{} > {}", number1, number2);
    // } else {
    //     println!("{} <= {}", number1, number2);
    // }

    // let minimum = if number1 < number2 { number1 } else { number2 }; // Don't forget the semi-colon here!
    // println!("minimum is: {}", minimum)

    // let mut a = 15;
    // let mut b = 40;
    // while b != 0 {
    //     let temp = b;
    //     b = a % b;
    //     a = temp;
    // }
    // println!("Greatest common divisor of 15 and 40 is: {}", a)

    /* declaring function */
    // fn max(a: i32, b: i32) -> i32 {
    //     // int a, int b returns int.
    //     if a > b {
    //         a
    //     } else {
    //         b
    //     }
    // } // don't need semi colon here

    // #[derive(Debug)] // You need this to be able to utilize {:?} to generate and be able to print a debug representation of the structure.
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // let point = Point { x: 24, y: 42 }; // you do need semi colon here since you need to suppress the return value of this EXPRESSION.
    // println!("({}, {})", point.x, point.y);
    // // println!("{}", point); will not work
    // println!("{:#?}", point); // will however, work. adding # like so: {:#?} will beautify the printout.

    // /* Using reference */
    // let p1 = Point { x: 1, y: 2 };
    // let p2 = &p1; // simply writing p2 = p1 moves the value rather than copying it. '&' will point p2 to the same memory location of p1.
    // println!("x: {}", p1.x); // Now it is a legal function.

    // fn print_point(point: &Point) {
    //     // assign Point to 'point' param of this function.
    //     println!("x: {}, y: {}", point.x, point.y);
    // }

    // print_point(&p1);
    // println!("{}", p1.x);

    /* Using clone types */
    // #[derive(Debug, Clone)]
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // fn print_point_sec(point: Point) {
    //     println!("x: {}, y: {}", point.x, point.y);
    // }
    // let pFirst = Point { x: 1, y: 2 };
    // let pSecond = pFirst.clone();
    // print_point_sec(pFirst.clone());
    // println!("{}", pFirst.x);

    /* Copy types */
    // simple integers can be copied
    // basic types implement a special marker, namely Copy.
    // let num1 = 42;
    // let num2 = num1;
    // println!("{}", num1);

    // #[derive(Clone, Copy)] // Copy requires Clone. This only works for value types that can use Copy
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // fn print_point(point: Point) {
    //     println!("x: {}, y: {}", point.x, point.y);
    // }
    // let p1 = Point { x: 1, y: 2 };
    // let p2 = p1;
    // print_point(p1);
    // println!("{}", p1.x)

    /* Mutable references */
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // fn inc_x(point: &mut Point) { // make it so that the reference is mutable
    //     point.x += 1;
    // }
    // let mut p1 = Point { x: 1, y: 2 };
    // inc_x(&mut p1);
    // println!("{}", p1.x);

    /* declaring methods */
    // impl Point {
    //     fn dist_from_origin(&self) -> f64 {
    //         // &self is like 'this' in other lang.
    //         let sum_of_squares = self.x.pow(2) + self.y.pow(2);
    //         (sum_of_squares as f64).sqrt() // sqrt() method is defined only on floating points.
    //     }
    // }

    impl Point {
        fn translate(&mut self, dx: i32, dy: i32) {
            // in this case the self is now mutable reference.
            self.x += dx;
            self.y += dy;
        }
    }

    /* Constructors */
    // Rust does not have natural constructors, but we can make one using new() static method.
    impl Point {
        fn new(x: i32, y: i32) -> Self {
            // in this example, it doesn't take &self as parameter.
            Self { x: x, y: y } // Self is TYPE of self value. We could've used Point.
        }
    }

    // when field name is the same as the value assigned, you omit a few things.
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
