use walkdir::DirEntry;

mod extract;

fn main() {
    let mut _current_entry: DirEntry;
    let _file_name = "86 - Eighty Six - S01E02.mkv";
    let b = "E:\\a_Projects\\AN\\";
    extract::iter_over_all_files(b);
}


