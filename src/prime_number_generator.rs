pub fn generate_prime_numbers(length: usize) -> Vec<u32>
{
    return prime_number_generator_u32(length);
}

fn prime_number_generator_u32(_length: usize) -> Vec<u32>
{
    let mut output: Vec<u32> = vec![2];
    let mut tmp = 2;
    let mut check = 0;
    
    while output.len() < _length {
        tmp = tmp + 1;
        for &n in output.iter() {
            if tmp/n >= 2 && tmp%n == 0 {
                check = 1;
                break;
            }
        }
        if check == 0 {
            output.push(tmp);
            println!("current step{}: {}", output.len(), tmp);
        }
        check = 0;
    }
    output
}
