mod search;
mod sorts;

fn main() {
    sorts::bubble_sort::run();
    sorts::selection_sort::run();
    sorts::insertion_sort::run();
    search::binary_search::run();
}
