pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U> where
    F: FnMut(T) -> U
{
    // The input and output data types can differ, that's why is it's T for input and U for Output
    // FnMut is used because of counter that is mutable for test mutating_closure
    // Initially, Clone trait was used for T type, but the last test could not pass because of it.
    // The Clone trait accepts i32, f64 and String types, but not Struct types. It was removed.
    let mut res = vec![];

    for element in input {
        res.push(function(element))
    }

    res
}
