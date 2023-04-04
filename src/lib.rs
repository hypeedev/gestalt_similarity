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

        let mut s1_max_start_index = 0;
        let mut s2_max_start_index = 0;
        let mut max_common_length = 0;

        for i in 0..s1.len() {
            let mut s1_start_index = i;
            let ch: char = s1.chars().nth(s1_start_index).unwrap();
            let mut s2_start_index = match s2.chars().position(|c| c == ch) {
                Some(i) => i,
                None => continue
            };

            loop {
                let s1_char = match s1.char_indices().nth(s1_start_index) {
                    Some(t) => t.1,
                    None => break
                };

                let s2_char = match s2.char_indices().nth(s2_start_index) {
                    Some(t) => t.1,
                    None => break
                };

                if s1_char != s2_char { break }

                s1_start_index += 1;
                s2_start_index += 1;
            }

            let length = s1_start_index - i;

            if length > max_common_length {
                s1_max_start_index = i;
                s2_max_start_index = s2_start_index - length;
                max_common_length = length;
            }
        }

        if max_common_length == 0 { continue }

        matches += max_common_length as u32;

        s1_max_start_index = s1.char_indices().nth(s1_max_start_index).unwrap().0;
        s2_max_start_index = s2.char_indices().nth(s2_max_start_index).unwrap().0;

        queue.push(&s1[0..s1_max_start_index]);
        queue.push(&s2[0..s2_max_start_index]);

        queue.push(&s1[s1_max_start_index + max_common_length..]);
        queue.push(&s2[s2_max_start_index + max_common_length..]);
    }

    api::result_double(context, (2.0 * matches as f64 / length as f64).into());

    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_gestaltsimilarity_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "similarity", 2, similarity, FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC)?;
    Ok(())
}
