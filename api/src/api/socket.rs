use socketioxide::extract::{Data, SocketRef};


pub async fn socket_handler(socket: SocketRef) {
    socket.on("subscribe", |socket: SocketRef, Data::<String>(room)| {
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });

    socket.on("unsubscribe", |socket: SocketRef| {
        let _ = socket.leave_all();
    });
}