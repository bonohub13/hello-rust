pub fn foobar() {
    let foobar_array = ("foo", "bar");

    for i in 0..100 {
        if i%6 == 0 {
            println!("step{}: {}{}", i, foobar_array.0, foobar_array.1);
        } else if i%2 == 0 {
            println!("step{}: {}", i, foobar_array.0);
        } else if i%3 == 0 {
            println!("step{}: {}", i, foobar_array.1);
        } else {
            println!("step{0}: {0}", i);
        }
    }
}