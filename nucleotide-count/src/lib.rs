use std::collections::HashMap;

pub fn count(c: char, dna: &str) -> usize {
    dna.chars().into_iter().filter(|x| x == &c).count()
}

pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
    let mut dna_map = HashMap::with_capacity(5);
    let dna_symbols = ['A', 'C', 'G', 'T'];
    for c in &dna_symbols {
        dna_map.insert(c.clone(), 0 as usize);
    }
    for c in dna.chars() {
        if dna_map.contains_key(&c) {
            let new_count: usize = *dna_map.get_mut(&c).unwrap() + 1;
            dna_map.insert(c.clone(), new_count);
        }
    }
    dna_map
}
