fn convert_to_i8(input: Vec<u8>) -> Vec<i8> {
    let mut output: Vec<i8> = Vec::with_capacity(input.len());

    for byte in input {
        // Convert each u8 byte to i8 and push it to the output vector
        output.push(byte as i8);
    }

    output
}

