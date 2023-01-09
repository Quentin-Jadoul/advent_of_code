use std::rc::Rc;

use crate::input;

pub fn day7() -> input::Result<()> {
    let contents = input::load_day_file("exemple.txt");

    // create a stucture to store the data
    // the stucture has a file name, a size, a parent and a list of children
    // the parent is an option because it can be null
    // the children is a vector because it can have multiple children
    struct TreeStructure {
        name: String,
        size: usize,
        parent: Option<Rc<TreeStructure>>,
        childrens: Option<Vec<Rc<TreeStructure>>>,
    }

    impl TreeStructure {
        fn new(name: String, size: usize, parent: Option<Rc<TreeStructure>>, childrens: Option<Vec<Rc<TreeStructure>>>) -> Self {
            TreeStructure {
                name,
                size,
                parent,
                childrens,
            }
        }
        // get parent
        fn get_parent(&self) -> Option<Rc<TreeStructure>> {
            self.parent
        }
        // get childrens
        fn get_childrens(&self) -> Option<Vec<Rc<TreeStructure>>> {
            self.childrens
        }

        // set size of the directory
        // the size of a directory is the sum of the size of its children
        fn set_size(&mut self) {
            if self.childrens.is_some() {
                let mut size = 0;
                for child in self.childrens.unwrap() {
                    size += child.size;
                }
                self.size = size;
            }
        }
    }

    let root_dir = Rc::new(TreeStructure::new(String::from("/"), 0, None, None ));
    let mut current_dir: Rc<TreeStructure> = root_dir;

    for line in contents.lines() {
        // check the first word of the line
        let mut iter = line.split_whitespace();
        if iter.next() == Some("$") {
            if iter.next() == Some("ls") {
            } else if iter.next() == Some("dir") {
                if iter.next() == Some("..") {
                    // change the current dir to the parent dir
                    current_dir = current_dir.get_parent().unwrap();
                } else {
                    let dir_name = iter.next().unwrap();
                    // change the current dir to the new dir
                    let childrens: &Vec<Rc<TreeStructure>>  = current_dir.childrens.unwrap().as_mut();
                    for child in childrens.iter() {
                        if child.name == dir_name {
                            current_dir = child.clone();
                        }
                    }
                }
            }
        } else if line.split_whitespace().next() == Some("dir") {
            let dir_name = iter.next().unwrap();
            current_dir.childrens.unwrap().push(Rc::new(TreeStructure::new(String::from(dir_name), 0, Some(current_dir.clone()), None)));
        } else {
            let size = iter.next().unwrap().parse::<usize>().unwrap();
            let file_name = iter.next().unwrap();
            // add file to the chidrens
            current_dir.childrens.unwrap().push(Rc::new(TreeStructure::new(String::from(file_name), size, Some(current_dir.clone()), None)));
        }
    }

    // while current dir is not the root dir go to the parent dir
    while current_dir.parent.is_some() {
        current_dir = current_dir.parent.unwrap();
    }

    // set the size of each directory
    // while a dir has no size set its size by going to children
    // when a dir has a size go to the parent
    while current_dir.size == 0 {
        if current_dir.childrens.is_some() {
            for child in current_dir.childrens.unwrap() {
                if child.size == 0 {
                    current_dir = child;
                }
            }
        } else {
            current_dir.set_size();
            current_dir = current_dir.parent.unwrap();
        }
    }

    // while current dir is not the root dir go to the parent dir
    while current_dir.parent.is_some() {
        current_dir = current_dir.parent.unwrap();
    }

    let mut sum = 0;
    while current_dir.childrens.is_some() {
        for child in current_dir.childrens.unwrap() {
            if child.size < 100000 {
                sum += child.size;
            }
            current_dir = child;
        }
    }

    println!("Result: {}", sum);

    Ok(())
}