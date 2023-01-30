pub mod key;
pub mod utils;

/*
    AES
*/
pub fn aes(data: [u8; 16], key: &Vec<u8>, key_word_length: usize, rounds: usize) -> [u8; 16] {
    let mut state_matrix: [[u8; 4]; 4] = [[0; 4]; 4];

    for column in 0..4 {
        for row in 0..4 {
            state_matrix[row][column] = data[(4 * column) + row];
        }
    }

    let keys: Vec<[[u8; 4]; 4]> =
        crate::modules::aes::key::expand(key, key_word_length, rounds + 1);

    state_matrix = crate::modules::aes::utils::add_round_key(state_matrix, keys[0]);

    for i in 1..rounds {
        state_matrix = crate::modules::aes::utils::sub_bytes(state_matrix);
        state_matrix = crate::modules::aes::utils::shift_rows(state_matrix);
        state_matrix = crate::modules::aes::utils::mix_columns(state_matrix);
        state_matrix = crate::modules::aes::utils::add_round_key(state_matrix, keys[i]);
    }

    state_matrix = crate::modules::aes::utils::sub_bytes(state_matrix);
    state_matrix = crate::modules::aes::utils::shift_rows(state_matrix);
    state_matrix = crate::modules::aes::utils::add_round_key(state_matrix, keys[rounds]);

    let mut export_data: [u8; 16] = [0; 16];

    for column in 0..4 {
        for row in 0..4 {
            export_data[(4 * column) + row] = state_matrix[row][column];
        }
    }

    return export_data;
}
