const DEFAULT_OFFSET_AMOUNT: usize = 1;

pub struct Ground {
    pattern_chars: Vec<char>,
    offset: usize,
}

impl Ground {
    pub fn new(pattern: String) -> Self {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        Self {
            pattern_chars,
            offset: 0,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn get_visible_slice(&self, width: usize) -> String {
        let pattern_len = self.pattern_chars.len();
        if pattern_len == 0 || width == 0 {
            return String::new();
        }

        let start = self.offset % pattern_len;
        let mut visible = String::with_capacity(width);

        for i in 0..width {
            let index = (start + i) % pattern_len;
            visible.push(self.pattern_chars[index]);
        }

        visible
    }

    pub fn scroll(&mut self, amount: Option<usize>) {
        self.offset += amount.unwrap_or(DEFAULT_OFFSET_AMOUNT);
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_new_ground() {
        let pattern = "===".to_string();
        let ground = Ground::new(pattern.clone());
        assert_eq!(ground.pattern_chars, vec!['=', '=', '=']);
        assert_eq!(ground.offset, 0);
    }

    #[test]
    fn test_get_visible_slice() {
        let ground = Ground::new("-^-.-^.".to_string());
        assert_eq!(ground.get_visible_slice(5), "-^-.-");

        let ground2 = Ground::new("-^-.-^.".to_string());
        assert_eq!(ground2.get_visible_slice(10), "-^-.-^.-^-");
    }

    #[test]
    fn test_scroll() {
        let mut ground = Ground::new("-^-.-^.".to_string());
        ground.scroll(Some(1));
        assert_eq!(ground.offset, 1);

        let mut ground2 = Ground::new("-^-.-^.".to_string());
        ground2.scroll(None);
        assert_eq!(ground2.offset, 1);
    }
}
