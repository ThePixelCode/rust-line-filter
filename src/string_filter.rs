use crate::traits::LineFiltering;

pub struct StringFilter {
    pub lines: Vec<String>
}

impl LineFiltering for StringFilter {
    fn filter(&mut self, order: &bool) {
        if *order {
            self.lines.sort()
        }
        self.lines.dedup()
    }

    fn get_filtered_string(&mut self) -> String {
        let lines_count = self.lines.len();
        for i in 0..lines_count {
            self.lines.get_mut(i).unwrap().push('\n');
        }
        let lines_iter = self.lines.iter();
        String::from_iter(lines_iter.map(|x| x.as_str())).trim().to_string()
    }

    fn new(string_to_filter: String) -> Self {
        StringFilter { lines: string_to_filter.lines().map(|x| String::from(x)).collect() }
    }
}