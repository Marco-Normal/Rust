pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<T>,
    feature: Option<String>,
    threshold: Option<f64>,
    information_gain: Option<f64>,
}

pub struct DecisionTree<T> {
    root: Option<Box<Node<T>>>,
    min_samples_split: usize,
    max_depth: usize,
}
