use itertools::Itertools;
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::iter::repeat;
use velcro::{btree_set, hash_set};

type FileId = isize;
type Idx = usize;
type BlockSize = usize;
type DiskMap = Vec<BlockSize>;
type BlockMap = Vec<FileId>;
type FileMap = BTreeMap<FileId, (Idx, BlockSize)>;
type FreeSpaceMap = BTreeMap<BlockSize, BTreeSet<Idx>>;
const EMPTY_SPACE_ID: FileId = -1;

fn to_blocks(input: &DiskMap) -> (BlockMap, FileMap, FreeSpaceMap) {
    let mut blocks = BlockMap::new();
    let mut is_free_space = false;
    let mut idx = 0;
    for input_idx in 0..input.len() {
        let block_size = input[input_idx];
        if is_free_space {
            blocks.extend(repeat(EMPTY_SPACE_ID).take(block_size));
        } else if block_size > 0 {
            blocks.extend(repeat(idx).take(block_size));
            idx += 1;
        }
        is_free_space = !is_free_space;
    }
    let mut idx = 0;
    let mut free_space_map = FreeSpaceMap::new();
    let mut file_map = FileMap::new();
    for chunk in blocks.chunk_by(|x, y| x == y) {
        match chunk.first() {
            Some(&EMPTY_SPACE_ID) => {
                let chunk_size = chunk.len();
                if free_space_map.contains_key(&chunk_size) {
                    free_space_map.get_mut(&chunk_size).unwrap().insert(idx);
                } else {
                    free_space_map.insert(chunk.len(), btree_set![idx]);
                }
            }
            Some(file_id) => {
                file_map.insert(*file_id, (idx, chunk.len()));
            }
            None => panic!(),
        }
        idx += chunk.len();
    }

    (blocks, file_map, free_space_map)
}

fn next_left(blocks: &Vec<isize>, prev_left: usize) -> usize {
    prev_left
        + blocks
            .iter()
            .skip(prev_left)
            .position(|x| *x == -1)
            .unwrap()
}
fn next_right(blocks: &Vec<isize>, prev_right: usize) -> usize {
    prev_right
        + blocks
            .iter()
            .rev()
            .skip(prev_right)
            .position(|x| *x != -1)
            .unwrap()
}
fn defrag(blocks: &Vec<isize>) -> Vec<isize> {
    let mut defragged = blocks.clone();
    let mut left_pos: usize = next_left(&defragged, 0);
    let mut right_pos: usize = next_right(&defragged, 0);
    while left_pos + right_pos + 1 < blocks.len() {
        defragged.swap(left_pos, blocks.len() - 1 - right_pos);
        left_pos = next_left(&defragged, left_pos);
        right_pos = next_right(&defragged, right_pos);
    }
    defragged
}

fn _defrag_by_file(
    blocks: &BlockMap,
    _file_map: &FileMap,
    _free_space_map: &FreeSpaceMap,
) -> Vec<isize> {
    let mut blocks = blocks.clone();
    let mut already_moved: HashSet<FileId> = hash_set![EMPTY_SPACE_ID];
    'next_file: while !already_moved.contains(&0) {
        let copy_block = blocks.clone();
        let chunked_blocks = copy_block.chunk_by(|a, b| a == b).collect_vec();
        let mut right_idx = blocks.len();
        for file_chunk in chunked_blocks.iter().rev() {
            right_idx -= file_chunk.len();
            if file_chunk[0] != EMPTY_SPACE_ID && !already_moved.contains(&file_chunk[0]) {
                let mut left_idx = 0;
                for empty_chunk in chunked_blocks.iter() {
                    if left_idx >= right_idx {
                        already_moved.insert(file_chunk[0]);
                        break;
                    }
                    if empty_chunk[0] == EMPTY_SPACE_ID && empty_chunk.len() >= file_chunk.len() {
                        already_moved.insert(file_chunk[0]);
                        for i in 0..file_chunk.len() {
                            assert_ne!(blocks[right_idx + i], EMPTY_SPACE_ID);
                            blocks.swap(left_idx + i, right_idx + i);
                        }
                        continue 'next_file;
                    }
                    left_idx += empty_chunk.len()
                }
            }
        }
    }
    blocks
}
fn defrag_by_file(
    blocks: &BlockMap,
    file_map: &FileMap,
    free_space_map: &FreeSpaceMap,
) -> Vec<isize> {
    // let max_free_space = free_space_map.last_key_value().unwrap().0;
    let mut defragged = blocks.clone();
    let mut free_space_map = free_space_map.clone();
    for (_file_id, (file_idx, file_size)) in file_map.iter().rev() {
        let (free_idx_list, available_free_space_size) = {
            let mut leftmost_free_idx = Idx::MAX;
            let mut available_free_space_size = 0;
            for (free_space_size, free_idx_list) in free_space_map.iter() {
                let new_free_idx = free_idx_list.first().unwrap();
                if free_space_size >= file_size
                    && new_free_idx < file_idx
                    && new_free_idx < &leftmost_free_idx
                {
                    leftmost_free_idx = *new_free_idx;
                    available_free_space_size = *free_space_size;
                }
            }
            (
                free_space_map.get_mut(&available_free_space_size),
                available_free_space_size,
            )
        };
        if available_free_space_size == 0 {
            continue;
        }
        let free_idx_list = free_idx_list.unwrap();
        let move_to_idx = free_idx_list.pop_first().unwrap();
        assert!(move_to_idx < *free_idx_list.first().unwrap_or(&std::usize::MAX));
        for i in 0..*file_size {
            defragged.swap(move_to_idx + i, file_idx + i)
        }
        if free_idx_list.len() == 0 {
            free_space_map.remove(&available_free_space_size);
        }
        if available_free_space_size > *file_size {
            let new_size = available_free_space_size - file_size;
            let new_idx = move_to_idx + file_size;
            if !free_space_map.contains_key(&new_size) {
                free_space_map.insert(new_size, btree_set![new_idx]);
            } else {
                free_space_map.get_mut(&new_size).unwrap().insert(new_idx);
            }
            assert!(free_space_map.contains_key(&new_size));
            assert!(free_space_map.get(&new_size).unwrap().contains(&new_idx));
        }
    }
    defragged
}

fn checksum(blocks: &Vec<isize>) -> isize {
    blocks
        .iter()
        .enumerate()
        .map(|(x, y)| (x as isize) * max(0, *y))
        .sum()
}

fn process_input(input: &String) -> DiskMap {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn part1(input: &DiskMap) -> isize {
    let (blocks, _, _) = to_blocks(input);
    let defragged = defrag(&blocks);
    checksum(&defragged)
}

fn part2(input: &DiskMap) -> isize {
    let (blocks, file_map, free_space_map) = to_blocks(input);
    // check_free_space_map(&free_space_map);
    let defragged = defrag_by_file(&blocks, &file_map, &free_space_map);
    checksum(&defragged)
}

fn test() {
    {
        let example = process_input(&String::from("12345"));
        assert_eq!(example, vec![1, 2, 3, 4, 5]);

        let (blocks, file_map, free_space_map) = to_blocks(&example);

        assert_eq!(
            blocks,
            vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2]
        );
        assert_eq!(file_map.len(), 3);
        assert_eq!(file_map[&0], (0, 1));
        assert_eq!(file_map[&1], (3, 3));
        assert_eq!(file_map[&2], (10, 5));
        assert_eq!(free_space_map.len(), 2);
        assert_eq!(free_space_map.iter().nth(0).unwrap(), (&2, &btree_set![1]));
        assert_eq!(free_space_map.iter().nth(1).unwrap(), (&4, &btree_set![6]));
        assert_eq!(free_space_map[&2], btree_set![1]);
        assert_eq!(free_space_map[&4], btree_set![6]);

        let defragged = defrag(&blocks);
        assert_eq!(
            defragged,
            vec![0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1]
        );
    }
    {
        let pathological = process_input(&String::from("010101010101"));
        assert_eq!(pathological, vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
        let (blocks, file_map, free_space_map) = to_blocks(&pathological);
        assert_eq!(blocks, vec![-1, -1, -1, -1, -1, -1]);
        assert_eq!(file_map.len(), 0);
        assert_eq!(free_space_map.len(), 1);
        // assert_eq!(defrag(&blocks), blocks);
    }
    {
        let pathological = process_input(&String::from("11012003"));
        assert_eq!(pathological, vec![1, 1, 0, 1, 2, 0, 0, 3]);
        let (blocks, file_map, free_space_map) = to_blocks(&pathological);
        assert_eq!(blocks, vec![0, -1, -1, 1, 1, -1, -1, -1]);
        assert_eq!(file_map.len(), 2);
        assert_eq!(file_map[&0], (0, 1));
        assert_eq!(file_map[&1], (3, 2));
        assert_eq!(free_space_map.len(), 2);
        assert_eq!(free_space_map[&2], btree_set![1]);
        assert_eq!(free_space_map[&3], btree_set![5]);
        let defragged = defrag(&blocks);
        assert_eq!(defragged, vec![0, 1, 1, -1, -1, -1, -1, -1]);
    }
    {
        let demo = process_input(&String::from("2333133121414131402"));
        assert_eq!(
            demo,
            vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]
        );
        let (blocks, file_map, free_space_map) = to_blocks(&demo);
        assert_eq!(
            blocks,
            vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        );
        assert_eq!(file_map.len(), 10);
        assert_eq!(file_map[&0], (0, 2));
        assert_eq!(file_map[&1], (5, 3));
        assert_eq!(file_map[&2], (11, 1));
        assert_eq!(file_map[&3], (15, 3));
        assert_eq!(file_map[&4], (19, 2));
        assert_eq!(file_map[&5], (22, 4));
        assert_eq!(file_map[&6], (27, 4));
        assert_eq!(file_map[&7], (32, 3));
        assert_eq!(file_map[&8], (36, 4));
        assert_eq!(file_map[&9], (40, 2));
        assert_eq!(free_space_map.len(), 2);
        assert_eq!(free_space_map[&1], btree_set![18, 21, 26, 31, 35]);
        assert_eq!(free_space_map[&3], btree_set![2, 8, 12]);
        assert_eq!(
            _defrag_by_file(&blocks, &file_map, &free_space_map),
            vec![
                0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5,
                -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1
            ]
        );
        assert_eq!(part1(&demo), 1928);
        assert_eq!(part2(&demo), 2858);
    }
}

fn main() {
    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    let input = process_input(&input_str);

    test();

    let p1 = part1(&input);
    assert_eq!(p1, 6461289671426);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
    assert_eq!(p2, 6488291456470);
}
