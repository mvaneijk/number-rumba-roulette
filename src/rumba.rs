use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_random_rumba() -> String {
    // coordinates for blocks on the first row
    static x_rows: [i32; 3] = [50, 210, 370];
    static y_cols: [i32; 3] = [20, 120, 220];

    // coordinates for numbers on 1st, 2nd or 3rd column
    static one_x_coord: [i32; 3] = [105, 265, 425];
    static two_x_coord: [i32; 3] = [94, 254, 414];
    static three_x_coord: [i32; 3] = [92, 255, 412];
    static num_y_coord: [i32; 3] = [110, 210, 310];

    static colors: [&str; 9] = [
        "#ff0062",
        "#ff0062",
        "#ff0062",
        "#ffd000",
        "#ffd000",
        "#ffd000",
        "#0099ff",
        "#0099ff",
        "#0099ff"
    ];

    // 1 = r1, 2 = r2, 3 = r3
    // 4 = y1, 5 = y2, 6 = y3
    // 7 = b1, 8 = b2, 9 = b3
    let mut order: [usize; 9] = [9, 2, 4, 1, 7, 3, 5, 8, 6];
    let mut rng = thread_rng();
    order.shuffle(&mut rng);

    let mut count = 0usize;
    let mut rumba_html = String::new();
    rumba_html.push_str(BASE_RUMBA_START);

    for y in 0..=2 {
        for x in 0..=2 {
            let num = if order[count] % 3 != 0 { order[count] % 3 } else { 3 };
            let x_coord: i32;
            match num {
                1 => {x_coord = one_x_coord[x]}
                2 => {x_coord = two_x_coord[x]}
                3 => {x_coord = three_x_coord[x]}
                _ => {x_coord = 0; eprintln!("Error converting x_coord.")}
            }

            let add_html = format!("<rect x=\"{}\" y=\"{}\" rx=\"10\" ry=\"10\" width=\"150\" height=\"100\"
            style=\"fill:{};stroke:black;stroke-width:5;opacity:1\" />
            <text fill=\"black\" font-size=\"110\" font-family=\"Poppins\" x=\"{}\" y=\"{}\">{}</text>",
             x_rows[x], y_cols[y], colors[order[count]-1],
            x_coord, num_y_coord[y], num);

            rumba_html.push_str(&add_html.clone());
            count += 1;
        }
    }

    rumba_html.push_str(BASE_RUMBA_END);
    rumba_html
}

static BASE_RUMBA_START: &str = r#"

    <svg width="700" height="350">
        <!-- four black pillars -->
        <rect x="100" y="0" rx="20" ry="20" width="50" height="340"
        style="fill:black;stroke:black;stroke-width:5;opacity:1" />
        <rect x="260" y="0" rx="20" ry="20" width="50" height="340"
        style="fill:black;stroke:black;stroke-width:5;opacity:1" />
        <rect x="420" y="0" rx="20" ry="20" width="50" height="340"
        style="fill:black;stroke:black;stroke-width:5;opacity:1" />
        <rect x="580" y="0" rx="20" ry="20" width="50" height="340"
        style="fill:black;stroke:black;stroke-width:5;opacity:1" />
        <!-- bottom black bar -->
        <rect x="30" y="325" rx="0" ry="0" width="650" height="20"
        style="fill:black;stroke:black;stroke-width:5;opacity:1" />
"#;

static BASE_RUMBA_END: &str = r#"
        Sorry, your browser does not support inline SVG.
    </svg>

"#;