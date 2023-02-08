struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let piece = [
        // 長方形
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


    /*
    */

    let mut position = Position {
        x: 4,
        y: 0,
    };

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    for _ in 0..5 {
        let mut field_buf = field;

        for y in 0..4 {
            for x in 0..4 {
                field_buf[y+position.y][x+position.x] |= piece[0][y][x];
            }
        }

        position.y += 1;
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
    }
    // カーソルを再表示
    println!("\x1b[?25h");
}
