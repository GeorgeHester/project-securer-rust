use tauri;

#[tauri::command]
pub fn create_vault(user_id: String, vault_id: String) -> bool {
    let data_path: PathBuf = tauri::api::path::data_dir().expect("Error: Unable to access path");

    let mut vault_path: PathBuf = data_path.join(user_id);
    vault_path.set_file_name(format!("{}.psv", vault_id));

    let file = std::fs::File::create(vault_path).unwrap();

    return true;
}

pub fn write_vault(path: String, data: Vec<u8>) {
    let data_length: usize = data.len();
    //let number_of_chunks: usize = (data_length / 0xFFFFFFFF) + 1;

    let output: Vec<u8> = [0x00, 0x50, 0x53, 0x56, 0x00, 0x01].to_vec();
    let SDAT: Vec<u8> = [0x53, 0x44, 0x41, 0x54].to_vec();

    for i in 0..(data_length / 0xFFFFFFFF) + 1 {
        output.append(&mut SDAT);
        for j in 0..(data.len() - (i * 0xFFFFFFFF)) {}
    }
}
