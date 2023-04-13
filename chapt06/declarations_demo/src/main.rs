use std::io;
use std::cmp::Ordering;

fn show_files() -> io::Result<()> {
    let mut v = vec![];

    fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        a.timestamp.cmp(&b.timestamp) //first, compare timestamps
        .reverse()  // newest file first
        .then(a.path.cmp(&b.path)) // compare paths to break ties
    }

    v.sort_by(cmp_by_timestamp_then_name);

}

fn main() {
}
