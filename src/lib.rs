use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Result};

pub fn similarity(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let s1 = api::value_text(values.get(0).unwrap())?;
    let s2 = api::value_text(values.get(1).unwrap())?;

    let length: u32 = (s1.chars().count() + s2.chars().count()) as u32;
    let mut matches: u32 = 0;

    let mut queue: Vec<&str> = vec![s1, s2];
    while queue.len() != 0 {
        let s2 = queue.pop().unwrap();
        let s1 = queue.pop().unwrap();

        let mut max_index_1 = 0;
        let mut max_index_2 = 0;
        let mut max_length = 0;

        for i in 0..s1.len() {
            let mut index1 = i;
            let ch: char = s1.chars().nth(index1).unwrap();
            let mut index2 = match s2.chars().position(|c| c == ch) {
                Some(i) => i,
                None => continue
            };

            loop {
                let s1_char = match s1.char_indices().nth(index1) {
                    Some(t) => t.1,
                    None => break
                };

                let s2_char = match s2.char_indices().nth(index2) {
                    Some(t) => t.1,
                    None => break
                };

                if s1_char != s2_char { break }

                index1 += 1;
                index2 += 1;
            }

            let length = index1 - i;

            if length > max_length {
                max_index_1 = i;
                max_index_2 = index2 - length;
                max_length = length;
            }
        }

        if max_length == 0 { continue }

        matches += max_length as u32;

        max_index_1 = s1.char_indices().nth(max_index_1).unwrap().0;
        max_index_2 = s2.char_indices().nth(max_index_2).unwrap().0;

        queue.push(&s1[0..max_index_1]);
        queue.push(&s2[0..max_index_2]);

        queue.push(&s1[max_index_1 + max_length..]);
        queue.push(&s2[max_index_2 + max_length..]);
    }

    api::result_double(context, (2.0 * matches as f64 / length as f64).into());

    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_gestaltsimilarity_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "similarity", 2, similarity, FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC)?;
    Ok(())
}
