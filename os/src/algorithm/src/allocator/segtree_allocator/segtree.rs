//这里定义线段树的节点
use alloc::{vec, vec::Vec};
/// 线段树的节点
pub struct SegNode {
    left_child: usize,
    right_child: usize,
    value: bool,//false表示儿子们当中还有未被分配的节点，true表示儿子们当中的节点以及被分配完了
    lazy: bool,//lazy标记表示，每次插入节点的时候需要更新整个线段树到叶子节点。但是这样成本太高了，所以改为在需要更新到叶子节点的时候才修改
}

/// capacity:这个线段树可以表示[1,capacity]线段的使用情况
/// list是用于构造整个线段树的Vector
pub struct Segtree {
    capacity: usize,
    list: Vec<SegNode>
}
impl Segtree{
    //-----------private functions---------------
    //-----------public functions----------------
    /// 调用的时候应该先new，然后调用buildtree
    pub fn new(capacity: usize) -> Self {
        let mut v = Vec::new();
        for i in 1..4*capacity+2{
            let node = SegNode{
                left_child: 0,
                right_child: 0,
                value: false,
                lazy: false,
            };
            v.push(node);
        }
        Self {
            list: v,
            capacity: capacity,
        }
    }
    /// initlize tree
    /// i:节点编号
    /// left：当前节点的左孩子是谁
    /// right：当前节点的右孩子是谁
    /// root number: 1
    /// ATTENTION! TODO:waiting for checking......
    pub fn build_tree(&mut self,i:usize, left:usize, right:usize){
        self.list[i].left_child=left;
        self.list[i].right_child=right;
        self.list[i].value=false;
        if left==right {
            self.list[i].left_child=left;
            self.list[i].right_child=right;
            return
        }
        let mid=(left+right)/2;
        Segtree::build_tree(self,i*2,left,mid);
        Segtree::build_tree(self,i*2+1,mid+1,right);
    }

    //todo:拥有lazy标记
    fn change(&mut self,i:usize,left_node:usize,right_node:usize,is_allocated:bool)
    {
        let mid=(self.list[i].left_child+self.list[i].right_child)/2;
        //没有覆盖
        if self.list[i].right_child<left_node || right_node<self.list[i].left_child {
            return
        }
        //被当前区间覆盖
        else if left_node<=self.list[i].left_child && self.list[i].right_child<=right_node {
            self.list[i].value = is_allocated;
            //TODO:需要把所有的儿子们都标注成true
            //为了性能，这里改为delete某一个seg的时候再把这个标记push下去
            //因此线段树还需要一个lazy标记

            // 为了方便起见还是现在就更新到叶子节点吧
            // 应该不会出问题？
            if self.list[i].left_child != self.list[i].right_child {
                Segtree::change(self,i*2,left_node,mid,is_allocated);
                Segtree::change(self,i*2+1,mid+1,right_node,is_allocated);
            }
        }
        //仅在左子区间
        else if right_node<=mid {
            Segtree::change(self,i*2,left_node,right_node,is_allocated);
        }
        //仅在右子区间
        else if left_node>=mid+1 {
            Segtree::change(self,i*2+1,left_node,right_node,is_allocated); 
        }
        //分在左右区间
        else{
            Segtree::change(self,i*2,left_node,mid,is_allocated);
            Segtree::change(self,i*2+1,mid+1,right_node,is_allocated);
        }
    }
    pub fn insert(&mut self,i:usize,left_node:usize,right_node:usize){
        Segtree::change(self,i,left_node,right_node,true);
    }
    pub fn insert_node(&mut self,node:usize){
        Segtree::insert(self,1,node,node);
    }

    pub fn remove(&mut self,i:usize,left_node:usize,right_node:usize){
        Segtree::change(self,i,left_node,right_node,false);
    }
    pub fn remove_node(&mut self,node:usize){
        Segtree::remove(self,1,node,node);
    }

    //Todo:检查lazy标记
    pub fn get_value(&mut self,i:usize,left_node:usize,right_node:usize)->bool{
        let mid=(self.list[i].left_child+self.list[i].right_child)/2;
        //没有覆盖
        if self.list[i].right_child<left_node || right_node<self.list[i].left_child {
            return false
        }
        //被当前区间覆盖
        else if left_node<=self.list[i].left_child && self.list[i].right_child<=right_node {
            return self.list[i].value
            //TODO:需要把所有的儿子们都标注成true
            //为了性能，这里改为delete某一个seg的时候再把这个标记push下去
            //因此线段树还需要一个lazy标记

            // 为了方便起见还是现在就更新到叶子节点吧
            // 应该不会出问题？
        }
        //仅在左子区间
        else if right_node<=mid {
            return Segtree::get_value(self,i*2,left_node,right_node)
        }
        //仅在右子区间
        else if left_node>=mid+1 {
            return Segtree::get_value(self,i*2+1,left_node,right_node)
        }
        //分在左右区间
        else{
            return Segtree::get_value(self,i*2,left_node,mid)&&Segtree::get_value(self,i*2+1,mid+1,right_node)
        }
    }

    /// 在[left_node,right_node]区间内返回一个空的点的编号
    /// 如果有多个空点就返回编号比较大的那个
    /// 如果没有空点返回0
    fn find_node(&mut self,i:usize,left_node:usize,right_node:usize) -> usize{
        //如果到了叶子节点上
        if left_node==right_node {
            if self.list[i].value == false {//这个节点未被分配
                return left_node
            }else{
                return 0
            }
        }else{//否则就在非叶子节点上，在下面的两个区间返回编号比较大的就可以了
            let mid=(self.list[i].left_child+self.list[i].right_child)/2;
            let result = Segtree::find_node(self,i*2+1,mid+1,right_node);
            if result>0 {//如果递归一个就能找到大于0的就不用递归另一个了
                return result
            }else{
                return Segtree::find_node(self,i*2,left_node,mid)
            }
        }
    }

    pub fn find_a_node(&mut self) -> usize{
        return Segtree::find_node(self,1,1,self.capacity)
    }

    //--------------------check funtions-------------------------

    pub fn check_init(&mut self){
        for i in 1..4*self.capacity+1{
            //println!("i:{},left child:{},right child:{},is allocated:{}\n",i,self.list[i].left_child,self.list[i].right_child,self.list[i].value);
        }
    }
    pub fn check_value(&mut self){
        for i in 1..self.capacity+1{
            //println!("i:{},is allocated check:{}\n",
            //    i,Segtree::get_value(self,1,i,i));
        }
    }
}