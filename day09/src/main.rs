fn main() {
    let input_filename = "input";
    let input_filename = "demo";
    let input: Vec<u32> = std::fs::read_to_string(input_filename)
        .expect("Unable to read file")
        .chars()
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut curr_pos = 0;
    let mut tot_sum = 0;
    for (id, length) in input.iter().enumerate() {
        let sum = length * (curr_pos + curr_pos + length - 1) / 2;
        tot_sum += sum * id as u32;
        curr_pos += length;
    }
    println!("{}", tot_sum);
}
