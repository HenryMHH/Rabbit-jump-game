pub struct Ground {
    pattern: String,
    offset: usize,
}

impl Ground {
    pub fn new(pattern: String) -> Self {
        Self { pattern, offset: 0 }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    // 根據寬度取得可視的地面圖案片段
    pub fn get_visible_slice(&self, width: usize) -> String {
        let pattern_len = self.pattern.len();
        if pattern_len == 0 || width == 0 {
            return String::new();
        }
        let start = self.offset % pattern_len;

        // 優化字串拼接
        let mut visible = String::with_capacity(width);
        let chars: Vec<char> = self.pattern.chars().collect(); // 避免重複呼叫 chars().nth()

        for i in 0..width {
            let index = (start + i) % pattern_len;
            visible.push(chars[index]);
        }
        visible
    }

    pub fn scroll(&mut self) {
        self.offset += 1;
    }
}
