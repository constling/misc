pub async fn shutdown() {
    imp::shutdown().await
}

#[cfg(unix)]
mod imp {
    use tokio::signal::unix::{signal, SignalKind};
    use tracing::info;

    pub(super) async fn shutdown() {
        tokio::select! {
            // SIGINT  - To allow Ctrl-c to emulate SIGTERM while developing.
            () = sig(SignalKind::interrupt(), "SIGINT") => {}
            // SIGTERM - Kubernetes sends this to start a graceful shutdown.
            () = sig(SignalKind::terminate(), "SIGTERM") => {}
        };
    }

    async fn sig(kind: SignalKind, name: &'static str) {
        // Create a Future that completes the first
        // time the process receives 'sig'.
        signal(kind)
            .expect("Failed to register signal handler")
            .recv()
            .await;
        info!("received {}, starting shutdown", name,);
    }
}

#[cfg(not(unix))]
mod imp {
    use tracing::info;

    pub(super) async fn shutdown() {
        // On Windows, we don't have all the signals, but Windows also
        // isn't our expected deployment target. This implementation allows
        // developers on Windows to simulate proxy graceful shutdown
        // by pressing Ctrl-C.
        tokio::signal::windows::ctrl_c()
            .expect("Failed to register signal handler")
            .recv()
            .await;
        info!("received Ctrl-C, starting shutdown",);
    }
}
