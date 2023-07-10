
fn process_files(filename: Vec<String>) -> io::Result<()> {
    for document in filenames {
        let text = load(&document)?; // read source file
        let results = process(text); // compute statistics
        save(&document, results)?;   // write output file
    }
    Ok(())
}

use std::{thread, io};
use std::sync::Arc;

use rayon::prelude::*;

fn process_files_in_paralle(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()>
{
    filenames.par_iter()
        .map(|filename| process_file(filename, glossary))
        .reduce_with(|r1, r2| {
            if r1.is_err() { r1 } else { r2 }
        })
        .unwrap_or(Ok(()))
}

/*
fn process_files_in_parallel(filenames: Vec<String>,
                            glossary: Arc<GigabyteMap>)
    -> io::Result<()>
{
    // Divide the work into serveral chunks.
    const NTHREADS: usize = 8;
    let worklists = split_vec_into_chunks(filenames, NTHREADS);

    // Fork: Spawn a thread to handle each chunk.
    let mut thread_handles = vec![];
    for worklist in worklists {
        // This call to .clone() only clones the Arc and bumps the
        // reference count. It does not clone the GigabyteMap.
        let glossary_for_child = glossary.clone();
        thread_handles.push(
            spawn(move || process_files(worklist, &glossary_for_child))
        );
    }

    // Join: Wait for all threads to finish.
    for handle in thread_handles {
        handle.join().unwrap()?;
    }

    Ok(())

}
*/
fn main() {
    println!("Hello, world!");
}
