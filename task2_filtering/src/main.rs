fn main() {
    //create a collection (e.g., a vector) with some elements
    let numbers = vec![1, 2, 3, 4, 5];

    //initialize a FilterCondition object with the desired value
    let filter = FilterCondition {
        filter_type: 4,
    };

    //call the custom_filter function and store it in a new variable
    let result = custom_filter(numbers, &filter);

    //print the filtered result to the console
    println!("Filtered result is: {:?}", result);
}

//Define a struct called FilterCondition with a single field of the desired type for filtering.
struct FilterCondition {
    filter_type: i32,
}

//Implement a method called is_match on the FilterCondition struct that takes a reference to an
//item of the same type as the filter condition and returns a boolean indicating whether the
//item matches the condition.
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        &self.filter_type == item
    }
}

//The function iterates over the elements in the collection and returns a new collection
//containing only the elements that match the filter condition
fn custom_filter(collection: Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    collection
        .into_iter() //can't use numbers vector anymore
        .filter(|item| filter.is_match(item))
        .collect()
}
