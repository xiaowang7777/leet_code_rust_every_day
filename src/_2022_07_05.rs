use std::cell::RefCell;
use std::ops::DerefMut;

struct MyCalendar {
    node: std::cell::RefCell<TreeNode>,
}

struct CalenderNode(i32, i32);

impl CalenderNode {
    fn is_in(&self, start: i32, end: i32) -> bool {
        self.is_in1(start, end) || self.is_in2(start, end)
    }

    fn is_in1(&self, start: i32, end: i32) -> bool {
        (self.0 < start && self.1 > start) || (self.0 < end && self.1 > end)
    }

    fn is_in2(&self, start: i32, end: i32) -> bool {
        (self.0 > start && self.0 < end) || (self.1 > start && self.1 < end)
    }
}

struct TreeNode {
    left: *mut TreeNode,
    right: *mut TreeNode,

    data: Option<CalenderNode>,
}

impl TreeNode {
    fn new(start: i32, end: i32) -> TreeNode {
        TreeNode {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            data: Option::Some(CalenderNode(start, end)),
        }
    }

    fn new_with_empty() -> TreeNode {
        TreeNode {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            data: None,
        }
    }


    fn insert(root: &mut TreeNode, start: i32, end: i32) -> bool {
        if root.data.is_none() {
            root.data = Option::Some(CalenderNode(start, end));
            return true;
        }
        if let Some(data) = &root.data {
            if data.is_in(start, end) {
                return false;
            }
            if data.0 > start {
                if root.left.is_null() {
                    root.left = Box::into_raw(Box::new(TreeNode::new(start, end)));
                } else {
                    unsafe {
                        return TreeNode::insert(root.left.as_mut().unwrap(), start, end);
                    }
                }
            } else {
                if root.right.is_null() {
                    root.right = Box::into_raw(Box::new(TreeNode::new(start, end)));
                } else {
                    unsafe {
                        return TreeNode::insert(root.right.as_mut().unwrap(), start, end);
                    }
                }
            }
        }
        true
    }
    unsafe fn drop_self(&mut self) {
        if !self.right.is_null() {
            self.right.as_mut().unwrap().drop_self();
        }
        if !self.left.is_null() {
            self.left.as_mut().unwrap().drop_self();
        }
        if !self.right.is_null() {
            drop(Box::from_raw(self.right));
        }
        if !self.left.is_null() {
            drop(Box::from_raw(self.left));
        }
    }
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        unsafe {
            self.drop_self()
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            node: std::cell::RefCell::new(TreeNode::new_with_empty())
        }
    }

    fn book(&self, start: i32, end: i32) -> bool {
        TreeNode::insert(self.node.borrow_mut().deref_mut(), start, end)
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
mod test {
    use crate::_2022_07_05::MyCalendar;

    // 使用cargo miri test测试通过
    #[test]
    fn test() {
        {
            let obj = MyCalendar::new();
            assert_eq!(obj.book(10, 20), true);
            assert_eq!(obj.book(15, 25), false);
            assert_eq!(obj.book(20, 30), true);
        }
        {
            let obj = MyCalendar::new();
            assert_eq!(obj.book(37, 50), true);
            assert_eq!(obj.book(33, 50), false);
            assert_eq!(obj.book(4, 17), true);
            assert_eq!(obj.book(35, 48), false);
            assert_eq!(obj.book(8, 25), false);
        }
    }
}
