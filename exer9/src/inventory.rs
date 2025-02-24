#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct InventoryItem {
    count: u64,
    cost: f64,
    description: String,
}

impl InventoryItem {
    pub fn new(count: u64, cost: f64, description: String) -> InventoryItem {
        InventoryItem {
            count,
            cost,
            description,
        }
    }
}

impl Eq for InventoryItem {
    // f64 isn't technically fully orderable, but we'll live with it.
}
impl Ord for InventoryItem {
    // implement Ord so they can go in a BTreeSet
    fn cmp(&self, other: &InventoryItem) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl std::hash::Hash for InventoryItem {
    // implement Hash so they can go in a HashSet
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.count.hash(state);
        self.description.hash(state);
    }
}

pub fn total_value<'a>(items: impl IntoIterator<Item = &'a InventoryItem>) -> f64 {
    items
        .into_iter()
        .map(|item| item.count as f64 * item.cost)
        .sum()
}

pub fn out_of_stock<'a>(items: impl IntoIterator<Item = &'a InventoryItem>) -> Vec<InventoryItem> {
    items
        .into_iter()
        .filter(|item| item.count == 0)
        .cloned()
        .collect()
}

pub fn explode<'a>(items: impl IntoIterator<Item = &'a InventoryItem>) -> Vec<InventoryItem> {
    items
        .into_iter()
        .flat_map(|item| (0..item.count).map(move |_| InventoryItem::new(1, item.cost, item.description.clone())))
        .collect()
}
