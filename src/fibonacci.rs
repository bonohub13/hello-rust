pub fn generate_fibonacci() -> Vec<u32> {
    let mut output: Vec<u32> = vec![0, 1];

    for i in 0..46 {
        output.push(output[i] + output[i+1]);
    }
    output
}
