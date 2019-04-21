// Vectors

fn print_vector1(v: &mut [i32]) {
    for x in v.iter() {
        print!("{} ", x);
    }
    println!();
}

fn print_vector2(v: &mut [i32]) {
    for i in 0..v.len() {
        print!("{} ", v[i]);
    }
    println!();
}

fn main()
{
    // create a vector
    let mut v = vec![1, 2, 3, 4, 5];

    // vector length
    println!("Length: {}", v.len());
    print_vector1(&mut v);

    // add new element at the end
    v.push(10);

    println!("Length: {}", v.len());
    print_vector2(&mut v);

    // create a vector
    let mut a: Vec<i32> = Vec::new();

    // vector length
    println!("Length: {}", a.len());
    print_vector1(&mut a);

    // add elements in vector
    for i in 0..6 {
        a.push(i * 2);
    }

    println!("Length: {}", a.len());
    print_vector1(&mut a);

    // get first 5 elements
    a.truncate(5);
    print_vector1(&mut a);

    // get first 7 elements (> length so all elements are returned)
    a.truncate(7);
    print_vector1(&mut a);

    // delete last element 
    a.pop();
    print_vector1(&mut a);

    // insert element
    a.insert(1, 10);
    print_vector1(&mut a);

    // remove element by position O(n)
    a.remove(1);
    print_vector1(&mut a);

    // remove element by position (swap with last element) O(1)
    a.swap_remove(1);
    print_vector1(&mut a);
}