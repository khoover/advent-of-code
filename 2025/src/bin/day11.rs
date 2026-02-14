use anyhow::Result;
use aoc_2025::run_day;
use fnv::FnvHashMap;

type NodeId = [u8; 3];
type Graph = FnvHashMap<NodeId, Vec<NodeId>>;

const END: NodeId = [b'o', b'u', b't'];

fn part1(s: &str) -> Result<u64> {
    const START: NodeId = [b'y', b'o', b'u'];
    let graph = parse_graph(s);
    let mut paths_to_out: FnvHashMap<NodeId, u64> =
        FnvHashMap::with_capacity_and_hasher(graph.len(), Default::default());
    paths_to_out.insert(END, 1);
    Ok(recursive_path_search(&mut paths_to_out, &graph, START))
}

fn parse_graph(s: &str) -> Graph {
    s.lines()
        .filter(|line| line.len() > 5)
        .map(|line| {
            let dsts = line.as_bytes()[5..]
                .windows(3)
                .step_by(4)
                .map(|window| {
                    window
                        .try_into()
                        .expect("Windows didn't give a length 3 slice")
                })
                .collect::<Vec<_>>();
            (line.as_bytes()[..3].try_into().unwrap(), dsts)
        })
        .collect()
}

fn recursive_path_search(memo: &mut FnvHashMap<NodeId, u64>, graph: &Graph, start: NodeId) -> u64 {
    if let Some(&count) = memo.get(&start) {
        return count;
    }

    let count = graph
        .get(&start)
        .map(|neighbours| {
            neighbours
                .iter()
                .copied()
                .map(|node| recursive_path_search(memo, graph, node))
                .sum()
        })
        .unwrap_or_default();
    memo.insert(start, count);
    count
}

fn part2(s: &str) -> Result<u64> {
    const SERVER: NodeId = [b's', b'v', b'r'];
    const DAC: NodeId = [b'd', b'a', b'c'];
    const FFT: NodeId = [b'f', b'f', b't'];
    let graph = parse_graph(s);
    let mut memo_cache: FnvHashMap<NodeId, u64> =
        FnvHashMap::with_capacity_and_hasher(graph.len(), Default::default());

    memo_cache.insert(FFT, 1);
    let (to_fft_coeff, dac_start, end_start) = {
        let dac = recursive_path_search(&mut memo_cache, &graph, DAC);
        // For the problem to have an answer, either dac -> fft or fft -> dac is unreachable.
        // Will split into cases based on that.
        if dac != 0 {
            (dac, SERVER, FFT)
        } else {
            let svr = recursive_path_search(&mut memo_cache, &graph, SERVER);
            (svr, FFT, DAC)
        }
    };
    memo_cache.clear();

    memo_cache.insert(DAC, 1);
    let to_dac_coeff = recursive_path_search(&mut memo_cache, &graph, dac_start);
    memo_cache.clear();
    memo_cache.insert(END, 1);
    let to_end_coeff = recursive_path_search(&mut memo_cache, &graph, end_start);

    Ok(to_end_coeff * to_dac_coeff * to_fft_coeff)
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static PART1_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

#[test]
fn test_part1() {
    assert_eq!(part1(PART1_INPUT).unwrap(), 5);
}

static PART2_INPUT: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

#[test]
fn test_part2() {
    assert_eq!(part2(PART2_INPUT).unwrap(), 2);
}
