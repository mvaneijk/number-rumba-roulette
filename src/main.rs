// #![deny(warnings)]
use std::fs;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Users::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    // GET /chat -> websocket upgrade
    let chat = warp::path("chat")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users))
        });

    // GET / -> index html
    let html = fs::read_to_string("src/index_before_svg.html");
    let mut html = match html {
        Ok(html) => html,
        Err(e) => {
            eprintln!("couldn't read HTML");
            String::new()
        }
    };

    // coordinates for blocks on the first row
    static x_rows: [i32; 3] = [50, 210, 370];
    static y_cols: [i32; 3] = [20, 120, 220];

    // coordinates for numbers on 1st, 2nd or 3rd column
    static one_x_coord: [i32; 3] = [105, 265, 425];
    static two_x_coord: [i32; 3] = [94, 254, 414];
    static three_x_coord: [i32; 3] = [93, 256, 413];
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

            html.push_str(&add_html.clone());
            count += 1;
        }
    }

    let html2 = fs::read_to_string("src/index_after_svg.html");
    let html2 = match html2 {
        Ok(html) => html,
        Err(e) => {
            eprintln!("couldn't read HTML");
            String::new()
        }
    };

    html.push_str(&html2);

    let index = warp::path::end().map(move || warp::reply::html(html.clone()));

    let routes = index.or(chat);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn user_connected(ws: WebSocket, users: Users) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // Save the sender in our list of connected users.
    users.write().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Make an extra clone to give to our disconnection handler...
    let users2 = users.clone();

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &users).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users2).await;
}

async fn user_message(my_id: usize, msg: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}

static RED: &str = "#ff0062";
static YELLOW: &str = "#ffd000";
static BLUE: &str = "#0099ff";





