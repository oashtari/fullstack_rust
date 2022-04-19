pub fn say_hello() {
    println!("Hellow World!");
}

pub fn print(limit: u8) {
    let numbers = generate_sequence(limit);
    output_sequence(&numbers);
    // let vector_numbers = vec![1,2,3,4,5];
    // output_sequence(&vector_numbers);

    // let array_numbers: [u8; 5] = [1,2,3,4,5];
    // output_sequence(&array_numbers);
    // let () = numbers;
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
    // let mut numbers = Vec::new();
    // for n in 1..=limit {
    //     numbers.push(n);
    // };
    // numbers
}

fn output_sequence( numbers: &[u8] ) {
    for n in numbers.iter() {
        println!("{}", n);
    }

}

#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result,&[1,2,3]);
}