use iroh_bitswap::BitswapEvent;
use libp2p::{
    autonat, dcutr,
    gossipsub::GossipsubEvent,
    identify::Event as IdentifyEvent,
    kad::KademliaEvent,
    mdns::Event as MdnsEvent,
    ping::Event as PingEvent,
    relay,
    swarm::{behaviour::toggle::Toggle, dummy, NetworkBehaviour},
};

use super::peer_manager::PeerManagerEvent;

/// Event type which is emitted from the [`NodeBehaviour`].
///
/// [`NodeBehaviour`]: crate::behaviour::NodeBehaviour
#[derive(Debug)]
pub enum Event<B: NetworkBehaviour> {
    Ping(PingEvent),
    Identify(Box<IdentifyEvent>),
    Kademlia(KademliaEvent),
    Mdns(MdnsEvent),
    Bitswap(BitswapEvent),
    Autonat(autonat::Event),
    Relay(relay::v2::relay::Event),
    RelayClient(relay::v2::client::Event),
    Dcutr(dcutr::behaviour::Event),
    Gossipsub(GossipsubEvent),
    PeerManager(PeerManagerEvent),
    Custom(B::OutEvent),
}

impl<B: NetworkBehaviour> From<PingEvent> for Event<B> {
    fn from(event: PingEvent) -> Self {
        Event::Ping(event)
    }
}

impl<B: NetworkBehaviour> From<IdentifyEvent> for Event<B> {
    fn from(event: IdentifyEvent) -> Self {
        Event::Identify(Box::new(event))
    }
}

impl<B: NetworkBehaviour> From<KademliaEvent> for Event<B> {
    fn from(event: KademliaEvent) -> Self {
        Event::Kademlia(event)
    }
}

impl<B: NetworkBehaviour> From<MdnsEvent> for Event<B> {
    fn from(event: MdnsEvent) -> Self {
        Event::Mdns(event)
    }
}

impl<B: NetworkBehaviour> From<BitswapEvent> for Event<B> {
    fn from(event: BitswapEvent) -> Self {
        Event::Bitswap(event)
    }
}

impl<B: NetworkBehaviour> From<GossipsubEvent> for Event<B> {
    fn from(event: GossipsubEvent) -> Self {
        Event::Gossipsub(event)
    }
}

impl<B: NetworkBehaviour> From<autonat::Event> for Event<B> {
    fn from(event: autonat::Event) -> Self {
        Event::Autonat(event)
    }
}

impl<B: NetworkBehaviour> From<relay::v2::relay::Event> for Event<B> {
    fn from(event: relay::v2::relay::Event) -> Self {
        Event::Relay(event)
    }
}

impl<B: NetworkBehaviour> From<relay::v2::client::Event> for Event<B> {
    fn from(event: relay::v2::client::Event) -> Self {
        Event::RelayClient(event)
    }
}

impl<B: NetworkBehaviour> From<dcutr::behaviour::Event> for Event<B> {
    fn from(event: dcutr::behaviour::Event) -> Self {
        Event::Dcutr(event)
    }
}

impl<B: NetworkBehaviour> From<PeerManagerEvent> for Event<B> {
    fn from(event: PeerManagerEvent) -> Self {
        Event::PeerManager(event)
    }
}

// Implement this instance of the trait so that the Toggle<dummy::Behaviour>
// may be used when no custom behavior is desired.
impl From<void::Void> for Event<Toggle<dummy::Behaviour>> {
    fn from(event: void::Void) -> Self {
        Event::Custom(event)
    }
}
