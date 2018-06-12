use std::ops::Add;
use std::marker::PhantomData;

// the two units we represent (printable and passed by copy)
#[derive(Debug, Copy, Clone)]
struct Inch;
#[derive(Debug, Copy, Clone)]
struct Mm;

// Length have two type parameter (the unit is represented by a phantom type)
#[derive(Debug, Copy, Clone)]
struct Length<Unit, T>(T, PhantomData<Unit>);


// `T` is a type that must implement both traits `Copy` and the trait `Add<T, Output=T>`
// this block implements `Add<Length<Unit, T>` for `Length<Unit, T>`, 
// which means you can add a `Length` to another `Length` of the same type
impl<Unit, T: Add<T, Output=T> + Copy> Add<Length<Unit, T>> for Length<Unit, T> {
    type Output = Length<Unit, T>;

    fn add(self, rhs: Length<Unit, T>) -> Self::Output {
        let Length(ref left, _)  = self;
        let Length(ref right, _) = rhs;
        Length(*left + *right, PhantomData)
    }
}

fn main() {
    let one_foot:  Length<Inch, f32> = Length(12.0, PhantomData);
    let one_meter: Length<Mm, f32>   = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?}", two_feet);
    println!("one meter + one_meter = {:?}", two_meters);

    // compilation fail giving a type mismatch error
    // let one_feter = one_foot + one_meter;
}