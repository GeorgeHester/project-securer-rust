/*
    AES - Key Expansion - Step - Rotate Word
*/
pub fn rot_word(word: [u8; 4]) -> [u8; 4] {
    let mut new_word: [u8; 4] = [0; 4];

    for i in 0..4 {
        new_word[(i + 3) % 4] = word[i];
    }

    return new_word;
}

/*
    AES - Key Expansion - Step - Substitute Word
*/
pub fn sub_word(word: [u8; 4]) -> [u8; 4] {
    let mut new_word: [u8; 4] = [0; 4];

    for i in 0..4 {
        new_word[i] = crate::modules::aes::utils::s_box::forward(word[i]);
    }

    return new_word;
}
