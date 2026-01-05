//! LeetCode 212: Word Search II
//! Finds all words from a dictionary that can be formed by walking a board grid

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.word = Some(word.to_string());
    }
}

fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut root = TrieNode::default();
    for word in &words {
        root.insert(word);
    }

    let mut result = HashSet::new();
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            dfs(i, j, &mut visited, &board, &mut root, &mut result);
        }
    }
    result.into_iter().collect()
}

fn dfs(
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
    board: &[Vec<char>],
    node: &mut TrieNode,
    result: &mut HashSet<String>,
) {
    if visited[x][y] {
        return;
    }
    let c = board[x][y];
    if let Some(next) = node.children.get_mut(&c) {
        visited[x][y] = true;
        if let Some(w) = &next.word {
            result.insert(w.clone());
            next.word = None; // avoid duplicates
        }

        for (dx, dy) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && nx < board.len() as isize && ny < board[0].len() as isize {
                dfs(nx as usize, ny as usize, visited, board, next, result);
            }
        }
        visited[x][y] = false;
    }
}

fn main() {
    let board = vec![
        vec!['o','a','a','n'],
        vec!['e','t','a','e'],
        vec!['i','h','k','r'],
        vec!['i','f','l','v']
    ];
    let words = vec!["oath", "pea", "eat", "rain"]
        .into_iter().map(String::from).collect();
    let found = find_words(board, words);
    println!("Words found: {:?}", found);
}
