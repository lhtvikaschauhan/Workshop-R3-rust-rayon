use rayon::prelude::*;

// //-------------- parallel foreach
// fn main() {
//     let mut arr = [0, 7, 9, 11];
//     arr.par_iter_mut().for_each(|p| *p += 1);
//     println!("{:?}", arr);
// }

//------------ Test in parallel if any or all elements of a collection match a given predicate
fn main() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));
    //assert!(vec.par_iter().all(|n| *n >= 10));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));
}

//---------------- Search items using given predicate in parallel
// fn main() {
//     let v = vec![6, 2, 1, 9, 3, 8, 11];

//     let f1 = v.par_iter().find_any(|&&x| x == 9);
//     let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
//     let f3 = v.par_iter().find_any(|&&x| x > 8);
//     //let f4 = v.par_iter().find_any(|&&x| x > 11);

//     assert_eq!(f1, Some(&9));
//     assert_eq!(f2, Some(&8));
//     assert!(f3 > Some(&8));
//     //assert!(f4 > Some(&11));
// }

// --------------------- Map-reduce in parallel
// struct Person {
//     age: u32,
// }

// fn main() {
//     let v: Vec<Person> = vec![
//         Person { age: 23 },
//         Person { age: 19 },
//         Person { age: 42 },
//         Person { age: 17 },
//         Person { age: 17 },
//         Person { age: 31 },
//         Person { age: 30 },
//     ];

//     let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
//     let sum_over_30 = v
//         .par_iter()
//         .map(|x| x.age)
//         .filter(|&x| x > 30)
//         .reduce(|| 0, |x, y| x + y);

//     let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();

//     let avg_over_30 = sum_over_30 as f32 / num_over_30;
//     let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

//     assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
//     println!("The average age of people older than 30 is {}", avg_over_30);
// }
