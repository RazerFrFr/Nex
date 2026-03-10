use std::{io::Result, sync::Mutex};

use once_cell::sync::Lazy;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
    runtime::Runtime,
    sync::oneshot,
    task::JoinHandle,
};

struct ProxyRuntime {
    runtime: Runtime,
    shutdown: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

static PROXY: Lazy<Mutex<Option<ProxyRuntime>>> = Lazy::new(|| Mutex::new(None));

async fn handle_client(mut socket: TcpStream, target_addr: String) {
    if let Ok(mut target) = TcpStream::connect(target_addr).await {
        let _ = copy_bidirectional(&mut socket, &mut target).await;
    }
}

pub fn init(target_addr: String) -> Result<()> {
    kill_proxy();

    let runtime = Runtime::new().unwrap();
    let (tx, mut rx) = oneshot::channel();

    let task = runtime.spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:3551").await.unwrap();

        loop {
            tokio::select! {
                _ = &mut rx => {
                    break;
                }
                res = listener.accept() => {
                    if let Ok((stream, _)) = res {
                        let addr_clone = target_addr.clone();
                        tokio::spawn(handle_client(stream, addr_clone));
                    }
                }
            }
        }
    });

    *PROXY.lock().unwrap() = Some(ProxyRuntime {
        runtime,
        shutdown: tx,
        task,
    });

    Ok(())
}

pub fn kill_proxy() {
    if let Some(proxy) = PROXY.lock().unwrap().take() {
        let _ = proxy.shutdown.send(());
        let _ = proxy.task.abort();
        drop(proxy.runtime);
    }
}
