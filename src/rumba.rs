use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_random_rumba() -> String {
    // coordinates for blocks on the first row
    static X_ROWS: [i32; 3] = [50, 210, 370];
    static Y_COLS: [i32; 3] = [20, 120, 220];

    // coordinates for numbers on 1st, 2nd or 3rd column
    static ONE_X_COORD: [i32; 3] = [105, 265, 425];
    static TWO_X_COORD: [i32; 3] = [94, 254, 414];
    static THREE_X_COORD: [i32; 3] = [92, 255, 412];
    static NUM_Y_COORD: [i32; 3] = [110, 210, 310];

    static RED: &str = "#ff0062";
    static YELLOW: &str = "#ffd000";
    static BLUE: &str = "#0099ff";

    static COLORS: [&str; 9] = [
        RED, RED, RED,
        YELLOW, YELLOW, YELLOW,
        BLUE, BLUE, BLUE
    ];

    // 1 = r1, 2 = r2, 3 = r3
    // 4 = y1, 5 = y2, 6 = y3
    // 7 = b1, 8 = b2, 9 = b3
    let mut order: [usize; 9] = [9, 2, 4, 1, 7, 3, 5, 8, 6];
    let mut rng = thread_rng();
    order.shuffle(&mut rng);

    let mut count = 0usize;
    let mut rumba_html = String::new();

    for y in 0..=2 {
        for x in 0..=2 {
            let num = if order[count] % 3 != 0 { order[count] % 3 } else { 3 };
            let x_coord: i32;
            match num {
                1 => {x_coord = ONE_X_COORD[x]}
                2 => {x_coord = TWO_X_COORD[x]}
                3 => {x_coord = THREE_X_COORD[x]}
                _ => {x_coord = 0; eprintln!("Error converting x_coord.")}
            }

            let add_html = format!("<rect class=\"block\" x=\"{}\" y=\"{}\" rx=\"10\" ry=\"10\" width=\"150\" height=\"100\"
            style=\"fill:{};stroke:black;stroke-width:5;opacity:1\" />
            <text class=\"block\" fill=\"black\" font-size=\"110\" font-family=\"Poppins\" x=\"{}\" y=\"{}\">{}</text>",
             X_ROWS[x], Y_COLS[y], COLORS[order[count]-1],
            x_coord, NUM_Y_COORD[y], num);

            rumba_html.push_str(&add_html.clone());
            count += 1;
        }
    }

    rumba_html
}