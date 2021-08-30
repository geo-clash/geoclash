use super::{ClientPackets, ServerPackets, DeBin, SerBin};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::sync::{Arc};

// Write data back to the client
async fn socket_write(socket: &mut TcpStream, buf: &[u8]){
    println!("buf {:?}", buf);
    if let Err(e) = socket.write_all(buf).await {
        eprintln!("failed to write to socket; err = {:?}", e);
        return;
    }
}

async fn handle_client_packet<Db>(socket: &mut TcpStream, client_packet: ClientPackets, db: &Arc<Db>, evaluate: &fn(ClientPackets, &Arc<Db>) -> ServerPackets){
    socket_write(socket, &SerBin::serialize_bin(&evaluate(client_packet, db))).await;
}

// Handle a packet frm the client

pub async fn server<Db: std::marker::Sync + std::marker::Send + 'static>(
    addr: &'static str,
    evaluate: fn(ClientPackets, &Arc<Db>) -> ServerPackets,
    db: Arc<Db>,
) -> Result<(), Box<std::io::Error>> {
   
    let listener = TcpListener::bind(addr).await?;
	println!("Started server on {}...", addr);

    loop {
        let (mut socket, other_addr) = listener.accept().await?;

        // After getting a new connection first we see a clone of the database
                // being created, which is creating a new reference for this connected
                // client to use.
                let db = db.clone();

        tokio::spawn(async move {
            println!("Client connected from {}", other_addr);

            handle_client_packet(&mut socket, ClientPackets::Connect, &db, &evaluate).await;

            let mut buf = vec![];

            // In a loop, read data from the socket and write the data back.
            loop {
                match socket.read_to_end(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                match DeBin::deserialize_bin(&buf) {
                    Ok(client_packet) => {handle_client_packet(&mut socket, client_packet, &db, &evaluate).await;},
                    Err(e) => {
                        let err = format!("Invalid packet '{:?}'\tError: {}", String::from_utf8( buf.clone()).unwrap(), e);
                        eprintln!("{}", err);
                        socket_write(&mut socket,  err.as_bytes()).await;
                    }
                };

                
            }
        });
    }
}
