pub struct Directions {
    idx: usize,
}

impl Directions {
    pub fn new() -> Directions {
        Directions { idx: 0 }
    }
}

impl Iterator for Directions {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        const DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        if self.idx < DIRS.len() {
            let dir = DIRS[self.idx];
            self.idx += 1;
            Some(dir)
        } else {
            None
        }
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
        let dirs: Vec<(isize, isize)> = Directions::new().collect();
        assert_eq!(dirs, expected);
    }
}
