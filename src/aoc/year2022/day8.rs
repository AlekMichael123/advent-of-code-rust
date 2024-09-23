pub fn main(data: &str) {
  let tree_heights = parse_input(data);

  println!(
    "Part 1 -- Find number of trees visible from standing around the grid\n# of visible trees = {}",
    part1(&tree_heights),
  );

  println!(
    "Part 2 -- Find tree with highest scenic view value\n# of visible trees = {}",
    part2(&tree_heights),
  );
}

fn part1(tree_heights: &TreeHeights) -> u16 {
  TreeVisibility::calc_tree_visibilities(tree_heights)
    .iter()
    .map(|row| 
      row
        .iter()
        .map(|visibility| 
          match visibility {
            TreeVisibility::Visible => 1,
            TreeVisibility::NotVisible => 0,
          }
        )
        .sum::<u16>()
    )
    .sum()
}

fn part2(tree_heights: &TreeHeights) -> u32 {
  *TreeVisibility::calc_scenic_scores(tree_heights)
    .iter()
    .map(|row| row.iter().max().unwrap())
    .max()
    .unwrap()
}

fn parse_input(data: &str) -> TreeHeights {
  data
    .lines()
    .map(|line| 
      line
        .chars()
        .map(|c| c.to_digit(10).expect("parse_input: Cannot convert digit in input.") as u8)
        .collect()
    )
    .collect()
}

type TreeHeights = Vec<Vec<u8>>;
type TreeVisibilities = Vec<Vec<TreeVisibility>>;
type ScenicScores = Vec<Vec<u32>>;

#[derive(Clone)]
enum TreeVisibility {
  Visible,
  NotVisible,
}

impl TreeVisibility {
  fn calc_tree_visibilities(tree_heights: &TreeHeights) -> TreeVisibilities {
    let n = tree_heights.len();
    let m = tree_heights.first().expect("calc_tree_visibilities: tree_heights contains empty vectors").len();
    
    let mut tree_visibilities: TreeVisibilities = vec![vec![TreeVisibility::NotVisible; m]; n];
    
    // mark always visible
    tree_visibilities[0].fill(TreeVisibility::Visible); 
    tree_visibilities[n-1].fill(TreeVisibility::Visible);

    // for each row, check going from left and right until you cant see past a tree
    for i in 1..n-1 {
      // mark always visible
      tree_visibilities[i][0] = TreeVisibility::Visible;
      tree_visibilities[i][m-1] = TreeVisibility::Visible;

      let mut tallest = tree_heights[i][0];
      for j in 1..m-1 {
        let tree_height = tree_heights[i][j];
        if tallest < tree_height {
          tree_visibilities[i][j] = TreeVisibility::Visible;
          tallest = tree_height;
        }
      }
      let mut tallest = tree_heights[i][m-1];
      for j in (1..m-1).rev() {
        let tree_height = tree_heights[i][j];
        if tallest < tree_height {
          tree_visibilities[i][j] = TreeVisibility::Visible;
          tallest = tree_height;
        }
      }
    }
    // for each col, check going from top and bottom until you cant see past a tree
    for j in 1..m-1 {
      let mut tallest = tree_heights[0][j];
      for i in 1..n-1 {
        let tree_height = tree_heights[i][j];
        if tallest < tree_height {
          tree_visibilities[i][j] = TreeVisibility::Visible;
          tallest = tree_height;
        }
      }
      let mut tallest = tree_heights[n-1][j];
      for i in (1..n-1).rev() {
        let tree_height = tree_heights[i][j];
        if tallest < tree_height {
          tree_visibilities[i][j] = TreeVisibility::Visible;
          tallest = tree_height;
        }
      }
    }

    tree_visibilities
  }

  fn calc_scenic_scores(tree_heights: &TreeHeights) -> ScenicScores {
    let n = tree_heights.len();
    let m = tree_heights.first().expect("calc_tree_visibilities: tree_heights contains empty vectors").len();

    let mut scenic_scores: ScenicScores = vec![vec![0; m]; n];

    for i in 0..n {
      for j in 0..m {
        let comparable_tree_height = tree_heights[i][j];
        // try up
        let mut upwards_count: u32 = 0;
        for i in (0..i).rev() {
          let tree_height = tree_heights[i][j];
          upwards_count += 1;

          if tree_height >= comparable_tree_height {
            break;
          }
        }
        // try down
        let mut downards_count: u32 = 0;
        for i in i+1..n {
          let tree_height = tree_heights[i][j];
          downards_count += 1;

          if tree_height >= comparable_tree_height {
            break;
          }
        }
        // try left
        let mut leftward_count: u32 = 0;
        for j in (0..j).rev() {
          let tree_height = tree_heights[i][j];
          leftward_count += 1;

          if tree_height >= comparable_tree_height {
            break;
          }
        }
        // try right
        let mut rightward_count: u32 = 0;
        for j in j+1..m {
          let tree_height = tree_heights[i][j];
          rightward_count += 1;

          if tree_height >= comparable_tree_height {
            break;
          }
        }

        scenic_scores[i][j] = upwards_count * downards_count * leftward_count * rightward_count;
      }
    }

    scenic_scores
  }
}