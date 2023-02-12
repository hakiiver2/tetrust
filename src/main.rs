use std::{thread, time};
 use getch_rs::{Getch, Key};


const FIELD_WIDTH:  usize = 11 + 2;  // 壁
const FIELD_HEIGHT: usize = 20 + 1;  // 底
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];



 #[derive(Clone, Copy)]
enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

 type BlockShape = [[usize; 4]; 4];
 const BLOCKS: [BlockShape; 7] = [
     // Iブロック
     [
         [0,0,0,0],
         [0,0,0,0],
         [1,1,1,1],
         [0,0,0,0],
     ],
     // Oブロック
     [
         [0,0,0,0],
         [0,1,1,0],
         [0,1,1,0],
         [0,0,0,0],
     ],
     // Sブロック
     [
         [0,0,0,0],
         [0,1,1,0],
         [1,1,0,0],
         [0,0,0,0],
     ],
     // Zブロック
     [
         [0,0,0,0],
         [1,1,0,0],
         [0,1,1,0],
         [0,0,0,0],
     ],
     // Jブロック
     [
         [0,0,0,0],
         [1,0,0,0],
         [1,1,1,0],
         [0,0,0,0],
     ],
     // Lブロック
     [
         [0,0,0,0],
         [0,0,1,0],
         [1,1,1,0],
         [0,0,0,0],
     ],
     // Tブロック
     [
         [0,0,0,0],
         [0,1,0,0],
         [1,1,1,0],
         [0,0,0,0],
     ],
     ];

struct Position {
    x: usize,
    y: usize,
}

fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y+pos.y][x+pos.x] & BLOCKS[block as usize][y][x] == 1 {

                return true;
            }
        }
    }
    false
}

fn main() {

    let field = [
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1],
        ];

        let mut pos = Position { x: 4, y: 0 };
        let g = Getch::new();

        println!("\x1b[2J\x1b[H\x1b[?25l");


        loop {
            let mut field_buf = field;

            let new_pos = Position {
                x: pos.x,
                y: pos.y + 1,
            };

            if !is_collision(&field, &new_pos, BlockKind::I) {
                pos = new_pos;
            }

            for _y in 0..4 {
                for _x in 0..4 {
                    if BLOCKS[BlockKind::I as usize][_y][_x] == 1 {
                        field_buf[_y+pos.y][_x+pos.x] = 1;
                    }
                }
            }

            println!("\x1b[H");  // カーソルを先頭に移動


            for _y in 0..FIELD_HEIGHT {
                for _x in 0..FIELD_WIDTH {
                    if field_buf[_y][_x] == 1 {
                        print!("[]");
                    } else {
                        print!(" .");
                    }
                }
                println!();
            }
            thread::sleep(time::Duration::from_millis(1000));

            match g.getch() {
                Ok(Key::Left) => {
                    let new_pos = Position {
                        x: pos.x - 1,
                        y: pos.y,
                    };

                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        pos = new_pos;
                    }
                }
                Ok(Key::Down) => {
                    let new_pos = Position {
                        x: pos.x,
                        y: pos.y + 1,
                    };
                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        // posの座標を更新
                        pos = new_pos;
                    }
                }
                Ok(Key::Right) => {
                    let new_pos = Position {
                        x: pos.x + 1,
                        y: pos.y,
                    };
                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        // posの座標を更新
                        pos = new_pos;
                    }
                }

                Ok(Key::Char('q')) => break,
                _ => (),
            }

        }
        println!("\x1b[?25h");

}
