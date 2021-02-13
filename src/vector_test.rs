pub fn run() {
    let vector = vec![1, 2, 3, 4];

    println!("Find 2 in vector: {:?}", vector.iter().find(|&&x|x == 2));
}
