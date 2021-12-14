#[cfg(test)]
mod tests {
    use super::*;
    use hyper::rt;
    use jsonrpc_client_transports::transports::http::connect;
    use std::time::Duration;

    #[test]
    fn test_get_public_parameters() {
        let endpoint = "http://localhost:9933";
        let (tx, rx) = std::sync::mpsc::channel();
        let run = connect(&endpoint)
            .and_then(|client: RpcClient| {
                client.get_public_parameters().and_then(move |result| {
                    drop(client);
                    let _ = tx.send(result);
                    Ok(())
                })
            })
            .map_err(|e| println!("RPC Client error: {:?}", e));

        rt::run(run);

        let result = rx.recv_timeout(Duration::from_secs(3));
        assert!(result.is_err())
    }
}
