pub mod binary;
pub mod rcon;
pub mod s_box;

/*
   AES - Step - Add Round Key
*/
pub fn add_round_key(state_matrix: [[u8; 4]; 4], subkey_matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut new_state_matrix: [[u8; 4]; 4] = [[0; 4]; 4];

    for row in 0..4 {
        for column in 0..4 {
            new_state_matrix[row][column] = state_matrix[row][column] ^ subkey_matrix[row][column];
        }
    }

    return new_state_matrix;
}

/*
   AES - Step - Substitute Bytes
*/
pub fn sub_bytes(state_matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut new_state_matrix: [[u8; 4]; 4] = [[0; 4]; 4];

    for row in 0..4 {
        for column in 0..4 {
            new_state_matrix[row][column] =
                crate::modules::aes::utils::s_box::forward(state_matrix[row][column]);
        }
    }

    return new_state_matrix;
}

/*
   AES - Step - Shift Rows
*/
pub fn shift_rows(state_matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut new_state_matrix: [[u8; 4]; 4] = [[0; 4]; 4];

    for column in 0..4 {
        new_state_matrix[0][column] = state_matrix[0][column];
    }

    for row in 1..4 {
        for column in 0..4 {
            let new_column: usize = (column + (4 - row)) % 4;
            new_state_matrix[row][new_column] = state_matrix[row][column];
        }
    }

    return new_state_matrix;
}

/*
   AES - Step - Mix Columns
*/
pub fn mix_columns(state_matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut new_state_matrix: [[u8; 4]; 4] = [[0; 4]; 4];

    for column in 0..4 {
        let mut state_column: [u8; 4] = [0; 4];
        let mut state_column_copy: [u8; 4] = [0; 4];
        let mut state_column_multiplied: [u8; 4] = [0; 4];
        let mut high_bit: u8;

        for row in 0..4 {
            state_column[row] = state_matrix[row][column];
            state_column_copy[row] = state_column[row];

            high_bit = (state_column[row] >> 7) & 1;

            state_column_multiplied[row] = state_column[row] << 1;
            state_column_multiplied[row] ^= high_bit * 0x1B;
        }

        state_column[0] = state_column_multiplied[0]
            ^ state_column_copy[3]
            ^ state_column_copy[2]
            ^ state_column_multiplied[1]
            ^ state_column_copy[1];
        state_column[1] = state_column_multiplied[1]
            ^ state_column_copy[0]
            ^ state_column_copy[3]
            ^ state_column_multiplied[2]
            ^ state_column_copy[2];
        state_column[2] = state_column_multiplied[2]
            ^ state_column_copy[1]
            ^ state_column_copy[0]
            ^ state_column_multiplied[3]
            ^ state_column_copy[3];
        state_column[3] = state_column_multiplied[3]
            ^ state_column_copy[2]
            ^ state_column_copy[1]
            ^ state_column_multiplied[0]
            ^ state_column_copy[0];

        new_state_matrix[0][column] = state_column[0];
        new_state_matrix[1][column] = state_column[1];
        new_state_matrix[2][column] = state_column[2];
        new_state_matrix[3][column] = state_column[3];
    }

    return new_state_matrix;
}
