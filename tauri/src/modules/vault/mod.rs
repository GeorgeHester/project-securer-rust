use super::logger;
use core::panic;
use std::path::Path;
use std::{fs::File, path::PathBuf};
use tauri::{self, Config};

pub fn create_vault(vault_id: &str, data: Vec<u8>) {
    let local_data_path_result: Option<PathBuf> = tauri::api::path::local_data_dir();

    if local_data_path_result == None {
        logger::print_error("Unable to get local data path");
        return;
    }

    let mut local_data_path: PathBuf =
        local_data_path_result.unwrap_or(panic!("Unable to get local data path"));

    local_data_path.push("Project Securer");
    local_data_path.set_file_name(vault_id);
    local_data_path.set_extension("psv");

    let mut vault_file = File::create(local_data_path);

    match vault_file {
        Ok() => 
    }

    /*let local_data_path = match local_data_path_result {
        Ok(path) => path,
        Err() => panic("test"),
    }*/
}
