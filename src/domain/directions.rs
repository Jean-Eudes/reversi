pub struct Directions {
    idx: usize,
}

impl Default for Directions {
    fn default() -> Directions {
        Directions { idx: 0 }
    }
}

impl Iterator for Directions {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        static  DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let dir = DIRS.get(self.idx);
        self.idx += 1;
        dir.copied()

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_check_all_directions() {
        let expected = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let dirs: Vec<(isize, isize)> = Directions::default().collect();
        assert_eq!(dirs, expected);
    }
}
