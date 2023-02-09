use std::{thread, time};
use getch_rs::{Getch, Key};

#[derive(Clone, Copy)]
enum PieceKind {
    I,
    O,
    T,
    J,
    L,
}

const PIECE: [[[usize; 4]; 4]; 5] = [
    // I
    [
        [0,0,0,0],
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0]
    ],
    // 四角
    [
        [0,0,0,0],
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0]
    ],
    // 凸型
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,1,0,0],
        [1,1,1,0]
    ],
    // J型
    [
        [0,0,0,0],
        [1,0,0,0],
        [1,1,1,0],
        [0,0,0,0]
    ],
    // L型
    [
        [0,0,0,0],
        [0,0,1,0],
        [1,1,1,0],
        [0,0,0,0]
    ]
];

struct Position {
    x: usize,
    y: usize,
}

fn is_collision(field: &[[usize;14]], position: &Position, piece: PieceKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y+position.y][x+position.x] & PIECE[piece as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let field = [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    let mut position = Position {
        x: 4,
        y: 0,
    };

    let g = Getch::new();

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // for _ in 0..30 {
    loop {
        let mut field_buf = field;

        let new_position = Position {
            x: position.x,
            y: position.y + 1,
        };

        if !is_collision(&field, &new_position, PieceKind::I) {
            position = new_position;
        }

        for y in 0..4 {
            for x in 0..4 {
                field_buf[y+position.y][x+position.x] |= PIECE[PieceKind::I as usize][y][x];
            }
        }

        // position.y += 1;
        println!("\x1b[H");  // カーソルを先頭に移動

        for y in 0..26 {
            for x in 0..14 {
                if field_buf[y][x] == 1 {
                    if x == 0 || x == 13 {
                        print!("|");
                    } else if y == 0 {
                        print!("~~");
                    } else if y == 25 {
                        print!("__");
                    } else {
                        print!(" ■");
                    }
                } else {
                    print!("  ");
                }
            }
            println!();
        }
        thread::sleep(time::Duration::from_millis(300));

        match g.getch() {
            Ok(Key::Char('q')) => break,
            Ok(Key::Down) => {
                let new_position = Position {
                    x: position.x,
                    y: position.y + 1,
                };

                if !is_collision(&field, &new_position, PieceKind::I) {
                    // posの座標を更新
                    position = new_position;
                }
            }
             _ => (),  // 何もしない
        }
    }
    // カーソルを再表示
    println!("\x1b[?25h");
}
