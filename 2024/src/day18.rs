use super::*;

use std::collections::BinaryHeap;

#[aoc(day18, part1)]
pub fn part1(s: &str) -> u32 {
    let mut grid = [[false; 71]; 71];
    s.lines().take(1024).for_each(|line| {
        let (x, y) = line.split_once(",").unwrap();
        grid[x.parse::<usize>().unwrap()][y.parse::<usize>().unwrap()] = true;
    });

    struct HeapEntry {
        pos: (usize, usize),
        distance: u32,
    }

    impl std::cmp::Ord for HeapEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.distance.cmp(&other.distance).reverse()
        }
    }

    impl std::cmp::PartialOrd for HeapEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for HeapEntry {
        fn eq(&self, other: &Self) -> bool {
            self.distance == other.distance
        }
    }

    impl Eq for HeapEntry {}

    let mut distance = [[u32::MAX; 71]; 71];
    distance[0][0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(HeapEntry {
        pos: (0, 0),
        distance: 0,
    });
    loop {
        let entry = heap.pop().unwrap();
        if entry.pos == (70, 70) {
            return entry.distance;
        }
        let next_distance = entry.distance + 1;

        for neighbour in neighbours(entry.pos, &grid) {
            if next_distance < distance[neighbour.0][neighbour.1] {
                distance[neighbour.0][neighbour.1] = next_distance;
                heap.push(HeapEntry {
                    pos: neighbour,
                    distance: next_distance,
                });
            }
        }
    }
}

#[aoc(day18, part2)]
pub fn part2(s: &str) -> String {
    let bytes: Vec<(usize, usize)> = s
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();

    struct HeapEntry {
        pos: (usize, usize),
        distance: u32,
        heuristic: u32,
    }

    impl std::cmp::Ord for HeapEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (self.distance + 2 * self.heuristic)
                .cmp(&(other.distance + 2 * other.heuristic))
                .reverse()
                .then_with(|| self.distance.cmp(&other.distance).reverse())
        }
    }

    impl std::cmp::PartialOrd for HeapEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for HeapEntry {
        fn eq(&self, other: &Self) -> bool {
            self.distance == other.distance && self.heuristic == other.heuristic
        }
    }

    impl Eq for HeapEntry {}

    // invariant:
    // bytes[..=left] can reach the end
    // bytes[..=right] can't
    let mut left = 0;
    let mut right = bytes.len() - 1;
    let mut heap = BinaryHeap::new();
    while right - left > 1 {
        let mid = (left + right) / 2;
        heap.clear();
        let mut grid = [[false; 71]; 71];
        bytes[..=mid].iter().copied().for_each(|(x, y)| {
            grid[x][y] = true;
        });

        let mut distance = [[u32::MAX; 71]; 71];
        distance[0][0] = 0;
        heap.push(HeapEntry {
            pos: (0, 0),
            distance: 0,
            heuristic: 70 + 70,
        });

        'astar: loop {
            let Some(entry) = heap.pop() else {
                right = mid;
                break 'astar;
            };
            if entry.pos == (70, 70) {
                left = mid;
                break 'astar;
            }
            let next_distance = entry.distance + 1;

            for neighbour in neighbours(entry.pos, &grid) {
                if next_distance < distance[neighbour.0][neighbour.1] {
                    distance[neighbour.0][neighbour.1] = next_distance;
                    heap.push(HeapEntry {
                        pos: neighbour,
                        distance: next_distance,
                        heuristic: neighbour.0.abs_diff(70) as u32
                            + neighbour.1.abs_diff(70) as u32,
                    });
                }
            }
        }
    }

    let res = bytes[right];
    format!("{},{}", res.0, res.1)
}

fn neighbours(
    pos: (usize, usize),
    grid: &[[bool; 71]; 71],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(move |offset| {
            pos.0
                .checked_add_signed(offset.0)
                .filter(|v| *v < 71)
                .zip(pos.1.checked_add_signed(offset.1).filter(|v| *v < 71))
                .filter(|&(x, y)| !grid[x][y])
        })
}
