/*
    Convert 8-bit unsinged integer into an 8-bit binary array
*/
pub fn integer_to_binary(integer: u8) -> [u8; 8] {
    let mut binary: [u8; 8] = [0; 8];
    let mut i: usize = 0;
    let mut integer_copy: u8 = integer;

    while integer_copy > 0 {
        binary[i] = integer_copy % 2;
        integer_copy /= 2;
        i += 1;
    }

    binary.reverse();

    return binary;
}

/*
    Convert 4-bit binary array nibble into an 8-bit unsigned integer
*/
pub fn binary_to_integer(binary: [u8; 4]) -> u8 {
    let mut integer: u8 = 0;

    for i in 0..4 {
        if binary[i] == 1 {
            integer += 2_u8.pow((3 - i).try_into().unwrap());
        }
    }

    return integer;
}
