use log::info;
use socketioxide::extract::{Data, SocketRef};


pub async fn socket_handler(socket: SocketRef) {
    info!("socket connected");
    socket.on("subscribe", |socket: SocketRef, Data::<String>(room)| {
        info!("socket subscribed to {room}");
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });

    socket.on("unsubscribe", |socket: SocketRef| {
        let _ = socket.leave_all();
    });
}