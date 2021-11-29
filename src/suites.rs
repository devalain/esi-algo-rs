/// 1,2,3,2,1,2,3,2,1,2,3,2,1,...
pub struct Suite1 {
    index: isize,
    next: isize
}
impl Suite1 {
    pub fn new() -> Self {
        Self {
            index: 1,
            next: 1
        }
    }
}
impl Iterator for Suite1 {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next += self.index % 2 + (self.index + 1) % 4 - 2;
        self.index += 1;
        Some(current)
    }
}

/// 1,2,3,4,3,2,1,2,3,4,3,2,1,2,3,...
pub struct Suite2 {
    index: isize,
    f: isize
}
impl Suite2 {
    pub fn new() -> Self {
        Self {
            index: 1,
            f: 1
        }
    }
}
impl Iterator for Suite2 {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.f = if self.f == 1_000_000 { 10 } else { 10 * self.f };
        let gen = 123432 * self.f / 999999;
        Some(gen % 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_1_works() {
        let suite1 = Suite1::new();
        let seq = suite1.take(9).collect::<Vec<isize>>();
        assert_eq!(seq, [1,2,3,2,1,2,3,2,1]);
    }

    #[test]
    fn suite_2_works() {
        let suite2 = Suite2::new();
        let seq = suite2.take(24).collect::<Vec<isize>>();
        assert_eq!(seq, [1,2,3,4,3,2,1,2,3,4,3,2,1,2,3,4,3,2,1,2,3,4,3,2]);
    }
}