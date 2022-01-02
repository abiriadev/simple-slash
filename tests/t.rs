#[tokio::main]
#[test]
async fn q6515167176() {
    // let connect_addr =
    //     env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));

    // let url = url::Url::parse(&connect_addr).unwrap();

    // let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    // tokio::spawn(read_stdin(stdin_tx));

    // let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    // println!("WebSocket handshake has been successfully completed");

    // let (write, read) = ws_stream.split();

    // let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    // let ws_to_stdout = {
    //     read.for_each(|message| async {
    //         let data = message.unwrap().into_data();
    //         tokio::io::stdout().write_all(&data).await.unwrap();
    //     })
    // };

    // pin_mut!(stdin_to_ws, ws_to_stdout);
    // future::select(stdin_to_ws, ws_to_stdout).await;

    simple_slash::ws::main1().await;
}
