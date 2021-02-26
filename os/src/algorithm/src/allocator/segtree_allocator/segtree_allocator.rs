//! 提供栈结构实现的分配器 [`Segtree_allocator`]

use super::super::Allocator;
use alloc::{vec, vec::Vec};


/// 每一页有4096bit
/// Bitmap 中的位数（4K）
const BITMAP_SIZE: usize = 4096;

/// 线段树的节点
pub struct SegNode {
    left_child: usize,
    right_child: usize,
    value: bool,//false表示儿子们当中还有未被分配的节点，true表示儿子们当中的节点以及被分配完了
    lazy: bool,//lazy标记表示，每次插入节点的时候需要更新整个线段树到叶子节点。但是这样成本太高了，所以改为在需要更新到叶子节点的时候才修改
}

pub struct Segtree {
    list: Vec<SegNode>
}

impl Segtree{
    /// 调用的时候应该先new，然后调用buildtree
    fn new(capacity: usize) -> Self {
        let mut v = Vec::new();
        for i in 1..4*capacity+1{
            let node = SegNode{
                left_child: 0,
                right_child: 0,
                value: false,
            };
            v.push(node);
        }
        Self {
            list: v,
        }
    }
    /// initlize tree
    /// i:节点编号
    /// left：当前节点的左孩子是谁
    /// right：当前节点的右孩子是谁
    /// root number: 1
    /// ATTENTION! TODO:waiting for checking......
    fn build_tree(&mut self,i:usize, left:usize, right:usize){
        self.list[i].left_child=left;
        self.list[i].right_child=right;
        self.list[i].value=false;
        if(left==right){
            return
        }
        let mid=(left+right)/2;
        Segtree::build_tree(self,i*2,left,mid);
        Segtree::build_tree(self,i*2+1,mid+1,right);
    }
    fn test(){
        //println!("test pass!");
    }

    fn insert(&mut self,int i,int left_node,int right_node)
    {
        let mut mid=(self.list[i].left_child+self.list[i].right_child)/2;
        //没有覆盖
        if(self.list[i].right_child<left_node or right_node<self.list[i].left_child) return;
        //被当前区间覆盖
        else if(left_node<=self.list[i].left_child and self.list[i].right_child<=right_node){
            self.list[i].value = true;
            //TODO:需要把所有的儿子们都标注成true
            //为了性能，这里改为delete某一个seg的时候再把这个标记push下去
            //因此线段树还需要一个lazy标记

            /// 为了方便起见还是现在就更新到叶子节点吧
            /// 应该不会出问题？
            if(self.list[i].left_child != self.list[i].right_child){
                Segtree::insert(self,i*2,left_node,mid);
                Segtree::insert(self,i*2+1,mid+1,right_node);
            }
        }
        //仅在左子区间
        else if(right_node<=mid){
            Segtree::insert(self,i*2,left_node,right_node);
        }
        //仅在右子区间
        else if(left_node>=mid+1){
            Segtree::insert(self,i*2+1,left_node,right_node); 
        }
        else//分在左右区间
        {
            Segtree::insert(self,i*2,left_node,mid);
            Segtree::insert(self,i*2+1,mid+1,right_node);
        }
    }
}

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
/// 
pub struct SegtreeAllocator {
    list: Vec<SegNode>,
}

impl Allocator for SegtreeAllocator {
    //capacity表示一共有多少个可分配的页面
    fn new(capacity: usize) -> Self {
        // 这里对list做初始化，先填进去4*capacity个SegNode
        let mut v = Vec::new();
        for i in 1..4*capacity+1{
            let node = SegNode{
                left_child: 0,
                right_child: 0,
                value: false,
            };
            v.push(node);
        }
        // SegtreeAllocator::test();
        // //build_tree(1,1,capacity);
        // SegtreeAllocator::build_tree(1,1,capacity);
        Self {
            list: v,
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        // if let Some((start, end)) = self.list.pop() {
        //     if end - start > 1 {
        //         self.list.push((start + 1, end));
        //     }
        //     Some(start)
        // } else {
        //     None
        // }
        None
    }

    fn dealloc(&mut self, index: usize) {
        //self.list.push((index, index + 1));
    }
}