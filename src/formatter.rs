use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

pub fn get_permission_string(mode: u16, read: u16, write: u16, execute: u16) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }
    .to_string()
}

pub fn parse_permissions(mode: u16) -> String {
    let user = get_permission_string(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
    let group = get_permission_string(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
    let other = get_permission_string(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);

    [user, group, other].join("")
}

pub fn format_size(size: u64) -> String {
    if size < 1000 {
        return format!("{}B", size);
    }

    let suffix = vec!["K", "M", "G", "T", "P", "E", "Z", "Y"];
    let mut current_size = size as f64 / 1000 as f64;

    for s in suffix.iter() {
        if current_size < 10.0 {
            return format!("{:.1}{}", current_size - 0.0499 as f64, s);
        }

        if current_size < 1000.0 && current_size >= 10.0 {
            return format!("{:.1}{}", current_size, s);
        }

        current_size /= 1000.0
    }

    String::from("")
}
