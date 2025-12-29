pub struct Table {
    columns: Vec<Vec<String>>,
    col_widths: Vec<usize>,
}

impl Table {
    //New table
    pub fn new(columns: Vec<Vec<String>>) -> Self {
        let col_widths = columns
            .iter()
            .map(|col| col.iter().map(|s| s.len()).max().unwrap_or(0))
            .collect();

        Table{columns, col_widths}
    }

    //print table
    pub fn print(&self) {
        
        //Obtain the largest vector
        let max_row = self.columns.iter().map(|c| c.len()).max().unwrap_or(0);
        
        //
        //Print the table
        //
        for row in 0..max_row {
            for (col_i, column) in self.columns.iter().enumerate() {
                let cell = column.get(row).map(|s| s.as_str()).unwrap_or("");
                let width = self.col_widths[col_i];
                print!("|");
                print!(" {} ", center(cell, width));
            }
            println!("|");
            println!("+{}+", "-".repeat(2*max_row - 2 - max_row%2));
        }

    }
}

fn center(s: &str,width: usize) -> String {
    let len = s.len();
    if len >= width {
        return s.to_string();
    }

    let padding = width - len;
    let left = padding/2;
    let right = padding - left;

    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))

}
