//! LeetCode 980: Unique Paths III
//! Count all unique paths that visit every non-obstacle square exactly once

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut grid = grid;
    let mut start = (0, 0);
    let mut empty = 0;

    for i in 0..rows {
        for j in 0..cols {
            match grid[i][j] {
                0 => empty += 1,
                1 => start = (i, j),
                _ => {}
            }
        }
    }

    fn dfs(grid: &mut [Vec<i32>], x: usize, y: usize, remain: i32) -> i32 {
        if grid[x][y] == 2 {
            return if remain == -1 { 1 } else { 0 };
        }

        let mut paths = 0;
        let tmp = grid[x][y];
        grid[x][y] = -1;

        for (dx, dy) in DIRS {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid[0].len() as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] >= 0 {
                    paths += dfs(grid, nx, ny, remain - 1);
                }
            }
        }

        grid[x][y] = tmp;
        paths
    }

    dfs(&mut grid, start.0, start.1, empty)
}

fn main() {
    let grid = vec![
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 2, -1],
    ];
    let count = unique_paths_iii(grid);
    println!("Unique paths visiting every square: {}", count);
}
