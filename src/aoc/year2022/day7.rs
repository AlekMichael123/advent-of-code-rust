pub fn main(data: &str) {
  // create simulated filesystem, maybe couldve used less abstraction but I think the parts came out nice
  let filesystem = parse_input(data);

  println!(
    "Part 1 -- Find all directories in a filesystem that are at most 100,000 in size -- \n# of directories = {}",
    part1(&filesystem),
  );

  println!(
    "Part 2 -- Find smallest directory to delete to get at least 30000000 unused space -- \nSize of deleted dir = {}",
    part2(&filesystem),
  );
}

/// Calculate the size of each directory in the filesystem 
/// then remove all sizes that we don't care about. 
/// Then, add up the remaining directories.
fn part1(filesystem: &Directory) -> FileSize {
  Directory::calc_all_dir_sizes(filesystem)
    .iter()
    .filter(|size| **size <= 100_000)
    .sum()
}

/// Calculate the directory sizes then only keep the directories that could reach us
/// to the required unused memory space (30000000). Then, extract the smallest usable directory!
fn part2(filesystem: &Directory) -> FileSize {
  let total_unused_space = 70_000_000 - filesystem.calc_dir_size();
  let mut candidates: Vec<FileSize> = 
    Directory::calc_all_dir_sizes(filesystem)
      .into_iter()
      .filter(|size| total_unused_space + *size >= 30_000_000)
      .collect();
      
  candidates.sort_unstable(); // order doesn't matter, so using "sort_unstable" which is supposedly faster
  candidates[0]
}

fn parse_input(data: &str) -> Directory {
  let mut head = Directory::default(); // create new directory head
  let mut path = vec!["/".to_string()]; // path to keep track where we're at with the cd's
  data
    .lines()
    .for_each(|line| {
      let mut line = line.split_ascii_whitespace();
      let Some(first_token) = line.next() else { panic!("parse_input: Somehow a blank line is in the input. Check format.") };
      // yeah sure this is fine lol
      match first_token {
        "$" => match line.next().unwrap() {
          "cd" => match line.next().unwrap() {
            ".." => { path.pop(); }, // move up in tree
            cd_dir => match cd_dir { // move to directory 
              "/" => { path = vec!["/".to_string()]; }, // go back to top (only used once lol???)
              d => path.push(d.to_string()), // change to named directory
            },
          },
          _ => (),
        },
        "dir" => {
          let mut dir = &mut head; // go to "current" directory by following path
          for dir_name in &path {
            dir = dir.find_dir(dir_name.clone()).unwrap();
          }
          dir.mkdir(line.next().unwrap().to_string()); // create directory with name
        },
        size => {
          let mut dir = &mut head; // go to "current" directory by following path
          for dir_name in &path {
            dir = dir.find_dir(dir_name.clone()).unwrap();
          }
          dir.mkfile(size.parse().expect("parse_input: Parsing file size failed.")); // create file with size 
        },
      }
    });
  head
}

type FileSize = u32;

#[derive(Debug)]
struct Directory {
  name: String,
  inner_dirs: Vec<Directory>,
  inner_files: Vec<FileSize>,
}

impl Default for Directory {
  fn default() -> Self {
    Self { name: "/".to_string(), inner_dirs: Default::default(), inner_files: Default::default() }
  }
}

impl Directory {
  fn new(name: String) -> Self {
    Self { name, ..Default::default() }
  }

  /// create sub directory
  fn mkdir(&mut self, name: String) {
    self.inner_dirs.push(Directory::new(name));
  }

  /// create file in self
  fn mkfile(&mut self, size: FileSize) {
    self.inner_files.push(size);
  }

  /// find sub directory in self with name
  fn find_dir(&mut self, name: String) -> Option<&mut Directory> {
    if self.name == name {
      return Some(self);
    }
    self
      .inner_dirs
      .iter_mut()
      .find(|dir| *dir.name == name)
  }

  /// calculate total size in self
  fn calc_dir_size(&self) -> FileSize {
    let total_directory_size: FileSize = 
      self
        .inner_dirs
        .iter()
        .map(|dir| dir.calc_dir_size())
        .sum();
    let total_file_size: FileSize =
      self
        .inner_files
        .iter()
        .map(|size| size)
        .sum();
    
    total_directory_size + total_file_size
  }

  // calc all directory sizes starting at given directory
  fn calc_all_dir_sizes(directory: &Directory) -> Vec<FileSize> {
    let mut res = vec![];
    let mut left_to_do: Vec<&Directory> = vec![directory];

    while !left_to_do.is_empty() {
      let curr = left_to_do.pop().unwrap();
      res.push(curr.calc_dir_size());
      curr
        .inner_dirs
        .iter()
        .for_each(|c| left_to_do.push(c));
    }

    res
  }
}