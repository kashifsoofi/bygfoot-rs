mod store;
mod domain;

use store::store::FileStore;

#[no_mangle]
pub extern "C" fn save_hint_number(hint_number: i32) {
    println!("Saving lib.rs hint_number: {}", hint_number);
    let hints_store = FileStore::new().hints_store();
    hints_store.save_hint_number(hint_number);
}

#[no_mangle]
pub extern "C" fn load_hint_number() -> i32 {
    let hints_store = FileStore::new().hints_store();
    let hint_num = hints_store.load_hint_number();
    println!("Loading lib.rs hint_number: {}", hint_num);
    hint_num
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[cfg(target_family = "unix")]
//     fn test_os_is_unix() {
//         let result = os_is_unix();
//         assert!(result);
//     }

//     #[test]
//     #[cfg(target_family = "windows")]
//     fn test_os_is_unix() {
//         let result = os_is_unix();
//         assert!(!result);
//     }
// }
