use datastructures::segment_tree::SegmentTree;
use algorithms::quicksort;

// need to declare subfolders as modules as per https://stackoverflow.com/questions/58935890/how-to-import-from-a-file-in-a-subfolder-of-src
mod algorithms {
    pub mod quicksort;
}

mod datastructures {
    pub mod segment_tree;
}

fn main() {
    // quicksort::quicksort_wrapper();

    println!("Starting with segment tree!");
    let values = vec![1, 3, 5, 7, 9, 11];
    println!("values: {:?}", values);
    let n = values.len();
    let mut st = SegmentTree::new(values);

    let start = 1;
    let end = 3;
    println!("tree: {:?}", st.tree);
    println!(
        "Sum [{}, {}]: {}",
        start,
        end,
        st.sum(1, 0, n - 1, start, end)
    );
    st.update(1, 0, n-1, 2, 100);
    println!("tree: {:?}", st.tree);
    println!(
        "Sum [{}, {}]: {}",
        start,
        end,
        st.sum(1, 0, n - 1, start, end)
    );
}


