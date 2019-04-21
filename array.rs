// Arrays

fn print_array1(v: &mut [i32]) {
    for x in v.iter() {
        print!("{} ", x);
    }
    println!();
}

fn print_array2(v: &mut [i32]) {
    for i in 0..v.len() {
        print!("{} ", v[i]);
    }
    println!();
}

fn main()
{
    let mut a = [1, 2, 3, 4, 5];

    print_array1(&mut a);
    a[0] = 7;
    a[a.len() - 1] = 10;
    print_array2(&mut a);

    let mut b: [i32; 4] = [0; 4];
    print_array2(&mut b);

    println!("Done!");
}