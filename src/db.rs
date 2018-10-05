use pager::{PAGE_SIZE, Pager};

use byteorder::{ByteOrder, BigEndian};

const ROW_SIZE: usize = 4 + 32 + 256;
pub const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pager: Pager,
    pub num_row: usize,
}

impl Table {
    pub fn new() -> Table {
        let pager = Pager::new();
        // let num_row = (pager.file.metadata().unwrap().len() / ROW_SIZE as u64) as usize;
        let num_row = if pager.num_pages == 0 {0} else {if pager.num_pages ==1 {1} else {1000}};

        Table{pager, num_row}
    }

    pub fn close(&mut self) {
        self.pager.close();
    }

    pub fn insert_row(&mut self, row: Row) {
        let bytes = row.serialize();

        let page_index = self.num_row / ROWS_PER_PAGE;
        let page = self.pager.page_to_write(page_index);
        let row_index = (self.num_row % ROWS_PER_PAGE) * ROW_SIZE;
        for (i, byte) in bytes.into_iter().enumerate() {
            page[row_index + i] = byte;
        }
        self.num_row += 1;
    }

    pub fn read_row(&mut self, num_row: usize) -> Row {
        let page_index = num_row / ROWS_PER_PAGE;
        let page = self.pager.page_to_read(page_index);
        let row_index = (num_row % ROWS_PER_PAGE) * ROW_SIZE;
        let bytes = page[row_index..row_index+ROW_SIZE].to_vec();

        Row::deserialize(bytes)
    }

    pub fn print_table(&mut self) {
        for i in 0..self.num_row {
            let row = self.read_row(i);
            row.print_row();
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

    fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![0; ROW_SIZE];
        BigEndian::write_u32(&mut buf, self.id);
        Row::write_string(&mut buf, 4, 32, &self.username);
        Row::write_string(&mut buf, 36, 256, &self.email);

        buf
    }

    fn deserialize(buf: Vec<u8>) -> Row {
        let id = BigEndian::read_u32(&buf);
        let username = Row::read_string(&buf, 4, 32);
        let email = Row::read_string(&buf, 36, 256);
        Row {id, username, email}
    }

    fn write_string(buf: &mut Vec<u8>, pos: usize, max_len: usize, s: &String) {
        let bytes = s.as_bytes().to_owned();

        let mut i = 0;
        for b in bytes {
            buf[pos + i] = b;
            i += 1;
        }
        while i < max_len {
            buf[pos + i] = 0;
            i += 1;
        }
    }

    fn read_string(buf: &Vec<u8>, pos: usize, max_len: usize) -> String {
        let mut end = pos;
        while end < max_len + pos && buf[end] != 0  {
            end += 1;
        }
        let bytes = buf[pos..end].to_vec();

        String::from_utf8(bytes).unwrap()
    }
}
