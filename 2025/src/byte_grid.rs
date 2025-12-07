use std::ops::{Index, IndexMut};

use anyhow::{Result, anyhow};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_input_str(s: &str) -> Result<Self> {
        Grid::from_input_lines(s.lines())
    }

    pub fn from_input_lines<'a>(it: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        let mut lines = it.into_iter().peekable();
        let width = lines.peek().map_or(0, |l| l.len());
        let mut height = 0;
        let mut data: Vec<u8> =
            Vec::with_capacity(lines.size_hint().1.map_or(width, |x| x * width));
        for line in lines {
            if line.len() != width {
                return Err(anyhow!("Grid is ragged"));
            }
            height += 1;
            data.extend_from_slice(line.as_bytes());
        }
        Ok(Grid {
            data,
            height,
            width,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&u8> {
        if col >= self.width {
            return None;
        }
        self.data.get(row * self.width + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut u8> {
        if col >= self.width {
            return None;
        }
        self.data.get_mut(row * self.width + col)
    }

    pub fn col(&self, idx: usize) -> impl Iterator<Item = u8> + '_ {
        self.col_at_rows(idx, 0..self.height)
    }

    pub fn col_at_rows<'a, I, II>(
        &'a self,
        col_idx: usize,
        rows: II,
    ) -> impl Iterator<Item = u8> + 'a
    where
        I: Iterator<Item = usize> + 'a,
        II: IntoIterator<IntoIter = I>,
    {
        rows.into_iter().map(move |row| self[(row, col_idx)])
    }

    pub fn find(&self, val: u8) -> Option<(usize, usize)> {
        self.data
            .iter()
            .copied()
            .enumerate()
            .find(|(_, x)| *x == val)
            .map(|(idx, _)| (idx / self.width, idx % self.width))
    }
}

impl Index<usize> for Grid {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.width..(index + 1) * self.width]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.data[index * self.width..(index + 1) * self.width]
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &u8 {
        self.get(index.0, index.1).unwrap()
    }
}
