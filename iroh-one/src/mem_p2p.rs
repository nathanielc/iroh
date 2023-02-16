/// A p2p instance listening on a memory rpc channel.
use iroh_p2p::config::Config;
use iroh_p2p::{DiskStorage, Keychain, Node};
use iroh_rpc_types::p2p::P2pAddr;
use libp2p::swarm::{behaviour::toggle::Toggle, dummy, NetworkBehaviour};
use tokio::task;
use tokio::task::JoinHandle;
use tracing::error;

/// Starts a new p2p node, using the given mem rpc channel.
pub async fn start(rpc_addr: P2pAddr, config: Config) -> anyhow::Result<JoinHandle<()>> {
    start_with_custom_behavior(rpc_addr, config, None::<Toggle<dummy::Behaviour>>).await
}
/// Starts a new p2p node, using the given mem rpc channel.
pub async fn start_with_custom_behavior<B>(
    rpc_addr: P2pAddr,
    config: Config,
    custom_behaviour: Option<B>,
) -> anyhow::Result<JoinHandle<()>>
where
    B: NetworkBehaviour + Send,
    iroh_p2p::Event<B>: From<<B as NetworkBehaviour>::OutEvent>,
{
    let kc = Keychain::<DiskStorage>::new(config.key_store_path.clone()).await?;

    let mut p2p = Node::new(config, rpc_addr, kc, custom_behaviour).await?;

    // Start services
    let p2p_task = task::spawn(async move {
        if let Err(err) = p2p.run().await {
            error!("{:?}", err);
        }
    });

    Ok(p2p_task)
}
