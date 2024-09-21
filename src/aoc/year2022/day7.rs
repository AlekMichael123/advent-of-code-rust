pub fn main(data: &str) {
  // println!("{:#?}", parse_input(data));
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

fn part1(filesystem: &Directory) -> FileSize {
  Directory::calc_all_dir_sizes(filesystem)
    .iter()
    .filter(|size| **size <= 100_000)
    .sum()
}

fn part2(filesystem: &Directory) -> FileSize {
  let total_unused_space = 70_000_000 - filesystem.calc_dir_size();
  let mut candidates: Vec<FileSize> = 
    Directory::calc_all_dir_sizes(filesystem)
      .into_iter()
      .filter(|size| total_unused_space + *size >= 30_000_000)
      .collect();
      
  candidates.sort_unstable();
  candidates[0]
}

fn parse_input(data: &str) -> Directory {
  let mut head = Directory::default();
  let mut path = vec!["/".to_string()];
  data
    .lines()
    .for_each(|line| {
      let mut line = line.split_ascii_whitespace();
      let Some(first_token) = line.next() else { panic!("parse_input: Somehow a blank line is in the input. Check format.") };
      match first_token {
        "$" => match line.next().unwrap() {
          "cd" => match line.next().unwrap() {
            ".." => { path.pop(); },
            cd_dir => match cd_dir {
              "/" => { path = vec!["/".to_string()]; },
              d => path.push(d.to_string()),
            },
          },
          _ => (),
        },
        "dir" => {
          let mut dir = &mut head;
          for dir_name in &path {
            dir = dir.find_dir(dir_name.clone()).unwrap();
          }
          dir.mkdir(line.next().unwrap().to_string());
        },
        size => {
          let mut dir = &mut head;
          for dir_name in &path {
            dir = dir.find_dir(dir_name.clone()).unwrap();
          }
          dir.mkfile(line.next().unwrap().to_string(), size.parse().expect("parse_input: Parsing file size failed."));
        },
      }
    });
  head
}


#[derive(Debug)]
struct Directory {
  name: String,
  inner_dirs: Vec<Directory>,
  inner_files: Vec<NamedFile>, // TODO: adjust later
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

  fn mkdir(&mut self, name: String) {
    self.inner_dirs.push(Directory::new(name));
  }

  fn mkfile(&mut self, name: String, size: FileSize) {
    self.inner_files.push(NamedFile::new(name, size));
  }

  fn find_dir(&mut self, name: String) -> Option<&mut Directory> {
    if self.name == name {
      return Some(self);
    }
    self
      .inner_dirs
      .iter_mut()
      .find(|dir| *dir.name == name)
  }

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
        .map(|file| file.size)
        .sum();
    
    total_directory_size + total_file_size
  }

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

type FileSize = u128;

#[derive(Debug, Default)]
struct NamedFile {
  name: String,
  size: FileSize,
}

impl NamedFile {
  fn new(name: String, size: FileSize) -> Self {
    Self { name, size, }
  }
}