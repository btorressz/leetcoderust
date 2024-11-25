use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s_to_t = HashMap::new();
        let mut map_t_to_s = HashMap::new();
        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            if let Some(&mapped_ch) = map_s_to_t.get(&ch_s) {
                if mapped_ch != ch_t {
                    return false;
                }
            }
            if let Some(&mapped_ch) = map_t_to_s.get(&ch_t) {
                if mapped_ch != ch_s {
                    return false;
                }
            }
            map_s_to_t.insert(ch_s, ch_t);
            map_t_to_s.insert(ch_t, ch_s);
        }
        
        true
    }
}
