
#[derive(Debug)]
struct Entry {
    pub items:Vec<Option<String>>,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        for i in &self.items {
            if !other.items.contains(i) {
                return false;
            }
        }

        return true;
    }
}

// simplification: NO COLUMNS CAN REPEAT VALUES

fn pairs(vectors: Vec<Vec<&str>>) -> Vec<Entry> {
    // Result is a list of list of integer pairs.
    let mut result:Vec<Entry> = Vec::new();

    for v_pos in 0..vectors.len() {
        for e_pos in 0..vectors[v_pos].len() {
            for inner_v_pos in 0..vectors.len() {
                for inner_e_pos in 0..vectors[inner_v_pos].len() {
                    if v_pos != inner_v_pos {
                        let mut new_v:Vec<Option<String>> = Vec::with_capacity(vectors.len());
                        for _ in 0..vectors.len() {
                            new_v.push(Option::None);
                        }
                        new_v[v_pos] = Option::Some(vectors[v_pos][e_pos].to_string());
                        new_v[inner_v_pos] = Option::Some(vectors[inner_v_pos][inner_e_pos].to_string());
                        let new_e = Entry { items: new_v };
                        if !result.contains(&new_e) {
                            result.push(new_e);
                        }
                    } 
                }
            }
        }
    }


    result
}

fn null_entry(entries: &mut Vec<Entry>) -> Option<Entry> {
    for i in 0..entries.len() {
        let mut none = false;
   
        for item_pos in 0..entries[i].items.len() {
            if entries[i].items[item_pos].is_none() {
                none = true;
            }
        }

        if none {
            return Some(entries.swap_remove(i))
        }
    }

    Option::None
}

fn collapse(mut entries: Vec<Entry>) -> Vec<Entry> {
    for _ in 0..entries.len()*3 {
        // Take a null entry out of this
        let null_entr_opt = null_entry(&mut entries);

        if null_entr_opt.is_none() {
            break; // guess there are none left.
        }

        let null_entry = null_entr_opt.unwrap();

        // Find some place we can shove this entry onto.
        let mut found = false;
        for mut entry in &mut entries {
            // Determine if this entry matches
            let mut matches = true;
            for i in 0..null_entry.items.len() {
                if let Some(ref left) = null_entry.items[i] {
                    if let Some(ref right) = entry.items[i] {
                        if left != right {
                            matches = false;
                        }
                    }
                }
            }

            if matches {
                found = true;
                for i in 0..entry.items.len() {
                    if entry.items[i].is_none() {
                        entry.items[i] = null_entry.items[i].clone();
                    }
                }
                break;
            }
        }

        if !found {
            entries.push(null_entry);
        }        
    }

    entries
}

pub struct AllPairs {
    pub combination: usize,
    pub all_pairs: Vec<Vec<Option<String>>>
}

pub fn all_pairs(vectors: Vec<Vec<&str>>) -> AllPairs {
    let mut combinations = 1;
    for v in &vectors {
        combinations = combinations * v.len()
    }

    let p = pairs(vectors);
    let p = collapse(p);

    AllPairs {
        combination: combinations,
        all_pairs: p.into_iter().map(|x| x.items).collect()
    }
}