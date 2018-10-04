const ROW_SIZE: usize = 4 + 32 + 256;
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pub rows: Vec<Row>
}

impl Table {
    pub fn new() -> Table {
        return Table{rows: Vec::with_capacity(TABLE_MAX_ROWS)}
    }

    pub fn insert_row(&mut self, row: Row) {
        self.rows.push(row)
    }

    pub fn print_table(&self) {
        for row in &self.rows {
            row.print_row()
        }
    }
}

pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {
    fn print_row(&self) {
        println!("({}, {}, {})", self.id, self.username, self.email);
    }
}
