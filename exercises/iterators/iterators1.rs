// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: Create an iterator
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   
    // First `next()` returns Some(&"banana")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    // Step 2: Next element after "banana" is "custard apple"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    // Next `next()` returns Some(&"avocado")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // Step 3: Next element after "avocado" is "peach"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    // Next `next()` returns Some(&"raspberry")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // Step 4: No more elements left, should return None
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
