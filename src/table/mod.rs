//use crossterm::terminal::size;

pub struct Table {
    columns: Vec<Vec<String>>,
    col_widths: Vec<usize>,
    headers: Vec<(String, usize)>,
    header_widths: Vec<usize>
}

struct Padding {
    left: usize,
    right: usize
}

impl Table {
    //New table
    pub fn new(columns: Vec<Vec<String>>, headers: Vec<(String, usize)>) -> Self {
        let col_widths = columns
            .iter()
            .map(|col| col.iter().map(|s| s.len()).max().unwrap_or(0))
            .collect();

        let header_widths = headers
            .iter()
            .map(|s| s.0.len())
            .collect();


        Table{columns, col_widths, headers, header_widths}
    }

    //print table
    pub fn print(&self) {
        
        //Obtain the largest vector
        let max_row = self.columns.iter().map(|c| c.len()).max().unwrap_or(0);
            
        //
        //Print the table
        //

        //Print Headers
        let col_sum: usize = self.col_widths.iter().sum();
        let table_len = col_sum + self.columns.len() * 3 - 1;

        println!("+{}+", "-".repeat(table_len));
        
        let mut cursor = 0;
        let mut count = 0;
        for (header, &span) in self.headers.iter().map(|(h,s)| (h,s)) {
            let hcols: usize = self.col_widths[cursor .. cursor + span].iter().sum();
            let hwidth = self.header_widths[count];
            let padding = hcols + 3*span - hwidth - 1;
            let left = padding / 2;
            let right = padding - left;
            print!("|");
            print!("{}{}{}", " ".repeat(left), header, " ".repeat(right));
            count += 1;
            cursor += span;
        }
        println!("|");
        println!("+{}+", "-".repeat(table_len));


        //Print the columns
        for row in 0..max_row {
            for (col_i, column) in self.columns.iter().enumerate() {
                let cell = column.get(row).map(|s| s.as_str()).unwrap_or("");
                let width = self.col_widths[col_i];
                print!("|");
                print!(" {} ", center(cell, width));
            }
            println!("|");
            println!("+{}+", "-".repeat(table_len));
        }

    }
}

fn center(s: &str,width: usize) -> String {
    let len = s.len();
    let pad: usize;

    if len >= width {
        pad = len - width;
    }
    else {
        pad = width - len;
    }
    
    let left = pad/2;

    let padding = Padding {
        left,
        right: pad - left
    };

    format!("{}{}{}", " ".repeat(padding.left), s, " ".repeat(padding.right))

}
