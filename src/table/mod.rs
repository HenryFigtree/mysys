pub struct Table {
    columns: Vec<Vec<String>>,
    col_widths: Vec<usize>,
    headers: Vec<(String, usize)>,
}

impl Table {
    //New table
    pub fn new(columns: Vec<Vec<String>>, headers: Vec<(String, usize)>) -> Self {
        let col_widths = columns
            .iter()
            .map(|col| col.iter().map(|s| s.len()).max().unwrap_or(0))
            .collect();

        Table{columns, col_widths, headers}
    }

    //print table
    pub fn print(&self) {
        
        //Obtain the largest vector
        let max_row = self.columns.iter().map(|c| c.len()).max().unwrap_or(0);
        let offset: usize = if max_row % 2 == 0 {1} else {0};
        let offset_i = offset as isize;
        let header_count = self.headers.len();
        let largest_header = self.headers.iter().map(|h| h.1).max().unwrap_or(0);
        let col_num_mod = largest_header as isize - 2;
        let len_mod = 2*(header_count as isize - 1) + col_num_mod;
            
        //
        //Print the table
        //

        //Print Headers
        let col_sum: usize = self.col_widths.iter().sum();
        let table_len = col_sum + self.columns.len() * 2 + max_row%2;
        let table_len_i = table_len as isize;
        let table_len_sum = table_len_i + offset_i + len_mod;
        let table_len_mod = table_len_sum as usize;

        println!("+{}+", "-".repeat(table_len_mod));
        
        let mut cursor = 0;
        for (header, &span) in self.headers.iter().map(|(h,s)| (h,s)) {
            let hcols: usize = self.col_widths[cursor .. cursor + span].iter().map(|w| w + 2).sum();
            print!("|");
            print!(" {} ", center(header, hcols + span - 3));

            cursor += span;
        }
        println!("|");
        println!("+{}+", "-".repeat(table_len_mod));


        //Print the columns
        for row in 0..max_row {
            for (col_i, column) in self.columns.iter().enumerate() {
                let cell = column.get(row).map(|s| s.as_str()).unwrap_or("");
                let width = self.col_widths[col_i];
                print!("|");
                print!(" {} ", center(cell, width));
            }
            println!("|");
            println!("+{}+", "-".repeat(table_len_mod));
        }

    }
}

fn center(s: &str,width: usize) -> String {
    let len = s.len();
    let padding: usize;
    let left: usize;
    let right: usize;

    if len >= width {
        padding = len - width;
    }
    else {
        padding = width - len;
    }

    left = padding/2;
    right = padding - left;

    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))

}
