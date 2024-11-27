struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    //solve the problem
    red: u8,
    green: u8,
    blue: u8
}

// impl ColorRegularStruct {
//     fn new(red: u8, green: u8, blue: u8) -> Self {
//         assert!(red <= 255, "Red value must be between 0 and 255");
//         assert!(green <= 255, "Green value must be between 0 and 255");
//         assert!(blue <= 255, "Blue value must be between 0 and 255");
//         ColorRegularStruct { red, green, blue }
//     }
// }

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        //let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };
        //let green = ColorTupleStruct(0, 255, 0);
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
