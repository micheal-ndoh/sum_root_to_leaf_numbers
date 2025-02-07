
#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: f64,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    
    pub fn new(val: f64) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Stress;

impl Stress {
    pub fn sum_numbers(root: Option<Box<TreeNode>>) -> f64 {
        fn dfs(cur: &Option<Box<TreeNode>>, add: f64) -> f64 {
            match cur {
                None => 0.0,
                Some(node) => {
   let add = add * 10.0 + node.val;
       if node.left.is_none() && node.right.is_none() {
                        add
           } else {
                       dfs(&node.left, add) + dfs(&node.right, add)
                    }
                }
          }
      }

        dfs(&root, 0.0)
  }
}























