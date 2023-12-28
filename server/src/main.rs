pub mod game;

use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, SinkExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type SharedGameState = Arc<Mutex<game::GameState>>;

async fn handle_connection(
    peer_map: PeerMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
    shared_game_state: SharedGameState,
    piece: Option<game::Piece>,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (mut outgoing, incoming) = ws_stream.split();

    let initial_game_state = shared_game_state.lock().unwrap().clone();
    // let (tx2, rx2) = unbounded();
    // tx2.clone()
    //     .unbounded_send(Message::Text(
    //         serde_json::to_string(&initial_game_state).unwrap(),
    //     ))
    //     .unwrap();

    // let piece_msg = match piece {
    //     Some(piece) => Message::Text(serde_json::to_string(&piece).unwrap()),
    //     None => Message::Text("".to_string()),
    // };'
    outgoing
        .send(Message::Text(
            serde_json::to_string(&initial_game_state).unwrap(),
        ))
        .await
        .unwrap();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        let coordinates: game::Coordinates = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        let mut game_state = shared_game_state.lock().unwrap();
        if piece.is_some() && piece.unwrap() == game_state.turn {
            game_state.request_action(coordinates);
            let peers = peer_map.lock().unwrap();

            // We want to broadcast the message to everyone except ourselves.
            let broadcast_recipients = peers.iter().map(|(_, ws_sink)| ws_sink);

            for recp in broadcast_recipients {
                let message = Message::Text(serde_json::to_string(&game_state.clone()).unwrap());
                recp.unbounded_send(message).unwrap();
            }
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    let game_state = SharedGameState::new(Mutex::new(game::GameState::new()));
    let mut player_index = 0;
    while let Ok((stream, addr)) = listener.accept().await {
        let piece = match player_index {
            0 => Some(game::Piece::Nought),
            1 => Some(game::Piece::Cross),
            _ => None,
        };
        player_index += 1;
        tokio::spawn(handle_connection(
            state.clone(),
            stream,
            addr,
            game_state.clone(),
            piece,
        ));
    }

    Ok(())
}
