#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use winapi::um::winnt::{
    PSID, FILE_GENERIC_READ, FILE_GENERIC_EXECUTE, FILE_GENERIC_WRITE,
    FILE_ALL_ACCESS, SYSTEM_MANDATORY_LABEL_NO_WRITE_UP,
    SYSTEM_MANDATORY_LABEL_NO_READ_UP, SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP,
};
use windows_acl::acl::ACL;
use windows_acl::acl::ACLEntry;
use windows_acl::acl::AceType;
use windows_acl::helper::*;

struct ProtectedDirectory {
    directory_name: String,
    user: String,
    acl: ACL,
    sid: PSID,
    sid_string: String,
}

fn create_directory(directory_name: &str) -> std::io::Result<()> {
    std::fs::create_dir(directory_name)?;
    Ok(())
}

fn get_directory_acl(directory_name: &str) -> std::io::Result<ACL> {
    let acl = ACL::from_file_path(directory_name, false).unwrap();
    Ok(acl)
}

fn create_protected_directory(directory_name: String) -> ProtectedDirectory {
    let user = current_user().unwrap();
    create_directory(&directory_name).unwrap();
    let acl = get_directory_acl(&directory_name).unwrap(); 
    let sid = (name_to_sid(&user, None).unwrap()).as_ptr() as PSID;
    let sid_string = sid_to_string(sid).unwrap();

    let protected_directory = ProtectedDirectory {directory_name, user: user.to_string(), acl, sid, sid_string};
    return protected_directory;
}

fn print_information(protected_directory: ProtectedDirectory) {
    println!("Directory Name: {}", protected_directory.directory_name);
    println!("User: {}", protected_directory.user);
    println!("SID: {:?}", protected_directory.sid);
    println!("SID as string {}", protected_directory.sid_string);
    println!("ACL: {:?}", protected_directory.acl);
}

fn main() {
    let mut protected_directory = create_protected_directory(".\\testing".to_string());
    print_information(protected_directory);
}
