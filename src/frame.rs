pub type Frame = Vec<Vec<&'static str>>;

// TODO: fix the import issue these constants should come from lib.rs
const NUM_ROWS: usize = 20;
const NUM_COLS: usize = 40;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

