/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self where T: PartialOrd {
        let mut new_list = LinkedList::new();
        let mut a_current = list_a.start;
        let mut b_current = list_b.start;
    
        loop {
            match (a_current, b_current) {
                (Some(a_ptr), Some(b_ptr)) => {
                    let a_val = unsafe { &(*a_ptr.as_ptr()).val };
                    let b_val = unsafe { &(*b_ptr.as_ptr()).val };
    
                    if a_val <= b_val {
                        // 取出a节点
                        let next_a = unsafe { (*a_ptr.as_ptr()).next };
                        // 添加到新链表
                        if new_list.start.is_none() {
                            new_list.start = Some(a_ptr);
                            new_list.end = Some(a_ptr);
                        } else {
                            unsafe {
                                (*new_list.end.unwrap().as_ptr()).next = Some(a_ptr);
                            }
                            new_list.end = Some(a_ptr);
                        }
                        // 设置当前节点的next为None
                        unsafe {
                            (*a_ptr.as_ptr()).next = None;
                        }
                        a_current = next_a;
                    } else {
                        // 取出b节点
                        let next_b = unsafe { (*b_ptr.as_ptr()).next };
                        // 添加到新链表
                        if new_list.start.is_none() {
                            new_list.start = Some(b_ptr);
                            new_list.end = Some(b_ptr);
                        } else {
                            unsafe {
                                (*new_list.end.unwrap().as_ptr()).next = Some(b_ptr);
                            }
                            new_list.end = Some(b_ptr);
                        }
                        // 设置当前节点的next为None
                        unsafe {
                            (*b_ptr.as_ptr()).next = None;
                        }
                        b_current = next_b;
                    }
                }
                (Some(a_ptr), None) => {
                    // 处理剩余的a节点
                    while let Some(ptr) = a_current {
                        let next_a = unsafe { (*ptr.as_ptr()).next };
                        // 添加到新链表
                        if new_list.start.is_none() {
                            new_list.start = Some(ptr);
                            new_list.end = Some(ptr);
                        } else {
                            unsafe {
                                (*new_list.end.unwrap().as_ptr()).next = Some(ptr);
                            }
                            new_list.end = Some(ptr);
                        }
                        unsafe {
                            (*ptr.as_ptr()).next = None;
                        }
                        a_current = next_a;
                    }
                    break;
                }
                (None, Some(b_ptr)) => {
                    // 处理剩余的b节点
                    while let Some(ptr) = b_current {
                        let next_b = unsafe { (*ptr.as_ptr()).next };
                        // 添加到新链表
                        if new_list.start.is_none() {
                            new_list.start = Some(ptr);
                            new_list.end = Some(ptr);
                        } else {
                            unsafe {
                                (*new_list.end.unwrap().as_ptr()).next = Some(ptr);
                            }
                            new_list.end = Some(ptr);
                        }
                        unsafe {
                            (*ptr.as_ptr()).next = None;
                        }
                        b_current = next_b;
                    }
                    break;
                }
                (None, None) => break,
            }
        }
    
        // 更新新链表的长度
        new_list.length = list_a.length + list_b.length;
        new_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}