use std::cmp;

pub struct SegmentTree {
    pub tree: Vec<i32>,
}

impl SegmentTree {
    // A public constructor method
    pub fn build_tree(values: &Vec<i32>, tree: &mut Vec<i32>, v_idx: usize, tl: usize, tr: usize) {
        if tl == tr {
            tree[v_idx] = values[tl];
        } else {
            let tm = (tl + tr) / 2;
            SegmentTree::build_tree(values, tree, v_idx * 2, tl, tm);
            SegmentTree::build_tree(values, tree, v_idx * 2 + 1, tm + 1, tr);
            tree[v_idx] = tree[v_idx * 2] + tree[v_idx * 2 + 1];
        }
    }
    pub fn new(values: Vec<i32>) -> SegmentTree {
        let mut tree: Vec<i32> = vec![0; (4 * values.len()).try_into().unwrap()];
        SegmentTree::build_tree(&values, &mut tree, 1, 0, values.len() - 1);

        println!("{:?}", tree);

        SegmentTree {
            tree: tree,
        }
    }

    pub fn sum(
        &self,
        root_idx: usize,
        node_l: usize,
        node_r: usize,
        query_l: usize,
        query_r: usize,
    ) -> i32 {
        // sum of values between l and r
        // node_l, node_r are the boundaries of the segment tree at the current node
        // Algorithm:
        // At each step, if current window l > r we stop (have recursed far enough)
        //               if window bounds == node bounds, use that value. Node bounds computed on the fly
        //               else recurse. Left node l remains l, left node r is min of the child r computed on the fly and the query r (we could fall completely within the left node, in which case we go into the node with smaller r than it has)
        //               Right node l is then similarly max of child node l or query l. Right node r always stay the same.
        //               query_l will become larger than query_r when left child r becomes too small.
        // Run using st.sum(1, 0, n-1, query_start, query_end)
        // println!("At node with range {}, {}", node_l, node_r);
        if query_l > query_r {
            return 0;
        } else if query_l == node_l && query_r == node_r {
            return self.tree[root_idx];
        } else {
            let mid: usize = (node_l + node_r) / 2;
            let left_child_idx = 2 * root_idx;
            let right_child_idx = 2 * root_idx + 1;
            let left_child_r = mid;
            let right_child_l = mid + 1;

            return self.sum(
                left_child_idx,
                node_l,
                left_child_r,
                query_l,
                cmp::min(left_child_r, query_r),
            ) + self.sum(
                right_child_idx,
                right_child_l,
                node_r,
                cmp::max(right_child_l, query_l),
                query_r,
            );
        }
    }

    pub fn update(&mut self, root_idx: usize, node_l: usize, node_r: usize, update_idx: usize, val: i32, ) {
        // recurse to location to update
        if node_l == node_r {
            self.tree[root_idx] = val;
        } else {
            let mid = (node_l + node_r) / 2;
            // if update_idx in left child, go left, else go right
            if update_idx <= mid {
                self.update(2*root_idx, node_l, mid, update_idx, val);
            } else {
                self.update(2*root_idx+1, mid+1, node_r, update_idx, val);
            }
            // update node to sum of its children
            self.tree[root_idx] = self.tree[2*root_idx] + self.tree[2*root_idx+1];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SegmentTree;

    #[test]
    fn test_segment_tree_sum_update() {
        let values = vec![1, 3, 5, 7, 9, 11];
        let n = values.len();
        let mut st = SegmentTree::new(values);
        let start = 1;
        let end = 3;
        assert_eq!(st.sum(1, 0, n - 1, start, end), 15);
        st.update(1, 0, n-1, 2, 100);
        assert_eq!(st.sum(1, 0, n - 1, start, end), 110);
    }
}