use std::sync::{Arc, Mutex};
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

fn draw(field: &Field, pos: &Position) {
    // 描画用フィールドの生成
    let mut field_buf = field.clone();
    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                field_buf[y+pos.y][x+pos.x] = 1;
            }
        }
    }
    // フィールドを描画
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn main() {

    let field = Arc::new(Mutex::new([
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
        ]));

        let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));


        // 画面クリア
        println!("\x1b[2J\x1b[H\x1b[?25l");
        // フィールドを描画
        draw(&field.lock().unwrap(), &pos.lock().unwrap());



        {
            let pos = Arc::clone(&pos);
            let field = Arc::clone(&field);
            let _ = thread::spawn(move || {
                loop {
                    // 1秒間スリーブする
                    thread::sleep(time::Duration::from_millis(1000));
                    // 自然落下
                    let mut pos = pos.lock().unwrap();
                    let mut field = field.lock().unwrap();
                    let new_pos = Position {
                        x: pos.x,
                        y: pos.y + 1,
                    };
                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        // posの座標を更新
                        *pos = new_pos;
                    } else {
                        // ブロックをフィールドに固定
                        for y in 0..4 {
                            for x in 0..4 {
                                if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                                    field[y+pos.y][x+pos.x] = 1;
                                }
                            }
                        }
                        // posの座標を初期値へ
                        *pos = Position { x: 4, y: 0 };
                    }
                    // フィールドを描画
                    draw(&field, &pos);
                }
            });
        }

        let g = Getch::new();

        loop {

            match g.getch() {
                Ok(Key::Left) => {
                    let mut pos = pos.lock().unwrap();
                    let mut field = field.lock().unwrap();

                    let new_pos = Position {
                        x: pos.x - 1,
                        y: pos.y,
                    };

                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        *pos = new_pos;
                    }
                    draw(&field, &pos);


                }
                Ok(Key::Down) => {
                    let mut pos = pos.lock().unwrap();
                    let mut field = field.lock().unwrap();

                    let new_pos = Position {
                        x: pos.x,
                        y: pos.y + 1,
                    };
                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        // posの座標を更新
                        *pos = new_pos;
                    }
                    draw(&field, &pos);

                }

                Ok(Key::Right) => {
                    let mut pos = pos.lock().unwrap();
                    let mut field = field.lock().unwrap();
                    let new_pos = Position {
                        x: pos.x + 1,
                        y: pos.y,
                    };
                    if !is_collision(&field, &new_pos, BlockKind::I) {
                        // posの座標を更新
                        *pos = new_pos;
                    }
                    draw(&field, &pos);

                }

                Ok(Key::Char('q')) => {
                    // カーソルを再表示
                    println!("\x1b[?25h");
                    return;
                }

                _ => (),
            }

        }
}
