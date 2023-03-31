

pub trait LineFiltering {
    fn filter(&mut self);
    fn get_filtered_string(&mut self) -> String;
    fn new(content_to_filter: String) -> Self;
}