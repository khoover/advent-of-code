use super::*;
use partitions::PartitionVec;

#[aoc(day12, part1)]
pub fn part1(s: &str) -> u64 {
    let columns = s.find("\n").unwrap();
    let stride = columns + 1;
    let rows = (s.len() + 1) / stride;
    let input = s.as_bytes();

    let mut regions: PartitionVec<u64> = PartitionVec::with_capacity(input.len());
    regions.resize(columns, 1);

    {
        // First row
        regions[0] = 2;
        for i in 0..columns - 1 {
            if input[i] != input[i + stride] {
                regions.push(1);
                regions[i] += 1;
            } else {
                regions.push(0);
                regions.union(i, i + columns);
            }
            if input[i] != input[i + 1] {
                regions[i] += 1;
                regions[i + 1] += 1;
            } else {
                regions.union(i, i + 1);
            }
        }
        regions[columns - 1] += 1;
        if input[columns - 1] != input[columns - 1 + stride] {
            regions.push(1);
            regions[columns - 1] += 1;
        } else {
            regions.push(0);
            regions.union(columns - 1, columns * 2 - 1);
        }
    }

    {
        // Middle rows
        for row in 1..rows - 1 {
            let input_base = row * stride;
            let region_base = row * columns;
            regions[region_base] += 1; // left edge of grid
            for i in 0..columns - 1 {
                // Assumption: left and top edges have been accounted of.
                if input[i + input_base] != input[i + input_base + stride] {
                    regions.push(1);
                    regions[i + region_base] += 1;
                } else {
                    regions.push(0);
                    regions.union(region_base + i, region_base + i + columns);
                }
                if input[input_base + i] != input[input_base + i + 1] {
                    regions[region_base + i] += 1;
                    regions[region_base + i + 1] += 1;
                } else {
                    regions.union(region_base + i, region_base + i + 1);
                }
            }
            regions[region_base + columns - 1] += 1;
            if input[input_base + columns - 1] != input[input_base + columns - 1 + stride] {
                regions.push(1);
                regions[region_base + columns - 1] += 1;
            } else {
                regions.push(0);
                regions.union(region_base + columns - 1, region_base + 2 * columns - 1);
            }
        }
    }

    {
        // Final row
        let input_base = (rows - 1) * stride;
        let region_base = (rows - 1) * columns;
        regions[region_base] += 1; // left edge
        for i in 0..columns - 1 {
            regions[region_base + i] += 1; // bottom edge
            if input[input_base + i] != input[input_base + i + 1] {
                regions[region_base + i] += 1;
                regions[region_base + i + 1] += 1;
            } else {
                regions.union(region_base + i, region_base + i + 1);
            }
        }
        regions[region_base + columns - 1] += 2;
    }

    regions
        .all_sets()
        .map(|set| {
            let (area, perimeter) = set.fold((0, 0), |(area, perimeter), (idx, edges)| {
                debug!(idx);
                debug!(edges);
                (area + 1, perimeter + edges)
            });
            debug!(area);
            debug!(perimeter);
            area * perimeter
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { part2_impl(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_impl(s: &str) -> u64 {
    let columns = s.find("\n").unwrap();
    let stride = columns + 1;
    let rows = (s.len() + 1) / stride;
    let input = s.as_bytes();

    let mut regions: PartitionVec<()> = PartitionVec::with_capacity(input.len()); // is now count of corners at space
    regions.resize(columns, ());

    {
        // First row
        for i in 0..columns - 1 {
            regions.push(());
            if input[i] == input[i + stride] {
                regions.union(i, i + columns);
            }
            if input[i] == input[i + 1] {
                regions.union(i, i + 1);
            }
        }
        regions.push(());
        if input[columns - 1] == input[columns - 1 + stride] {
            regions.union(columns - 1, columns * 2 - 1);
        }
    }

    {
        // Middle rows
        for row in 1..rows - 1 {
            let input_base = row * stride;
            let region_base = row * columns;
            for i in 0..columns - 1 {
                regions.push(());
                // Assumption: left and top edges have been accounted of.
                if input[i + input_base] == input[i + input_base + stride] {
                    regions.union(region_base + i, region_base + i + columns);
                }
                if input[input_base + i] == input[input_base + i + 1] {
                    regions.union(region_base + i, region_base + i + 1);
                }
            }
            regions.push(());
            if input[input_base + columns - 1] == input[input_base + columns - 1 + stride] {
                regions.union(region_base + columns - 1, region_base + 2 * columns - 1);
            }
        }
    }

    {
        // Final row
        let input_base = (rows - 1) * stride;
        let region_base = (rows - 1) * columns;
        for i in 0..columns - 1 {
            if input[input_base + i] == input[input_base + i + 1] {
                regions.union(region_base + i, region_base + i + 1);
            }
        }
    }

    regions
        .all_sets()
        .map(|set| {
            let (area, perimeter) = set.fold((0, 0), |(area, perimeter), (idx, _)| {
                let neighbour_offsets = [
                    -(columns as isize) - 1,
                    -(columns as isize),
                    -(columns as isize) + 1,
                    -1,
                    1,
                    columns as isize - 1,
                    columns as isize,
                    columns as isize + 1,
                ];
                let neighbour_different = neighbour_offsets
                    .into_iter()
                    .map(|offset| {
                        let neighbour_idx = idx.wrapping_add_signed(offset);
                        neighbour_idx >= regions.len() || regions.other_sets(neighbour_idx, idx)
                    })
                    .fold(0_u8, |acc, is_different| (acc << 1) | (is_different as u8));

                let mut edges = match (neighbour_different & 0b01011010).count_ones() {
                    4 => 4,
                    3 => 2,
                    2 if (neighbour_different & 0b01000000 == 0)
                        ^ (neighbour_different & 0b00000010 == 0) =>
                    {
                        1
                    }
                    _ => 0,
                };
                edges += (neighbour_different & 0b11010000 == 0b10000000) as u64;
                edges += (neighbour_different & 0b01101000 == 0b00100000) as u64;
                edges += (neighbour_different & 0b00010110 == 0b00000100) as u64;
                edges += (neighbour_different & 0b00001011 == 0b00000001) as u64;

                (area + 1, perimeter + edges)
            });
            debug!(area);
            debug!(perimeter);
            area * perimeter
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_simple() {
        let s = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(s), 140);
    }

    #[test]
    fn test_part2_simple() {
        let s = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part2(s), 80);
    }
}
