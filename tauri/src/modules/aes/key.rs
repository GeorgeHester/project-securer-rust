pub mod utils;

/*
    AES - Step - Expand Key
*/
pub fn expand(key: &Vec<u8>, key_word_length: usize, rounds: usize) -> Vec<[[u8; 4]; 4]> {
    // Word length is the number of 32 bit words in the key
    // Each 32 bit word is stored in a 4 byte array
    // Each array is stored in a dynamic vector
    let mut key_word_matrix: Vec<[u8; 4]> = vec![[0; 4]; key_word_length];

    // Store each byte of the key into the corresponding entry in each word of the key matrix
    for word in 0..key_word_length {
        for byte in 0..4 {
            key_word_matrix[word][byte] = key[(word * 4) + byte];
        }
    }

    // Define vector of arrays to store the round keys
    let mut word_matrix: Vec<[u8; 4]> = vec![[0; 4]; 4 * rounds];

    // Loop through each word of the round keys
    for word_index in 0..(4 * rounds) {
        if word_index < key_word_length {
            word_matrix[word_index] = key_word_matrix[word_index];
        } else if word_index >= key_word_length && word_index % key_word_length == 0 {
            let rotated_word: [u8; 4] =
                crate::modules::aes::key::utils::rot_word(word_matrix[word_index - 1]);
            let mut substituted_word: [u8; 4] =
                crate::modules::aes::key::utils::sub_word(rotated_word);
            substituted_word[0] ^=
                crate::modules::aes::utils::rcon::rc((word_index / key_word_length) as u8) as u8;

            for byte in 0..4 {
                word_matrix[word_index][byte] =
                    substituted_word[byte] ^ word_matrix[word_index - key_word_length][byte];
            }
        } else if word_index >= key_word_length
            && key_word_length > 6
            && (word_index - 4) % key_word_length == 0
        {
            let substituted_word: [u8; 4] =
                crate::modules::aes::key::utils::sub_word(word_matrix[word_index - 1]);

            for byte in 0..4 {
                word_matrix[word_index][byte] =
                    word_matrix[word_index - key_word_length][byte] ^ substituted_word[byte];
            }
        } else {
            for byte in 0..4 {
                word_matrix[word_index][byte] = word_matrix[word_index - key_word_length][byte]
                    ^ word_matrix[word_index - 1][byte];
            }
        }
    }

    let mut keys: Vec<[[u8; 4]; 4]> = vec![[[0; 4]; 4]; rounds];

    for round in 0..rounds {
        for word in 0..4 {
            for byte in 0..4 {
                keys[round][byte][word] = word_matrix[(round * 4) + word][byte];
            }
        }
    }

    return keys;
}
