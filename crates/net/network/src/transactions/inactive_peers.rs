use crate::NetworkHandle;
use futures::Stream;
use pin_project::pin_project;
use reth_network_api::NetworkInfo;
use reth_primitives::PeerId;
use std::{
    collections::{HashMap, VecDeque},
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

#[pin_project]
#[derive(Debug)]
pub(crate) struct InactivePeers {
    inactivity_threshold: std::time::Duration,
    network: NetworkHandle,
    last_activity: HashMap<PeerId, Instant>,
    inactive_peers: VecDeque<PeerId>,
    #[pin]
    interval: tokio::time::Interval,
}

impl InactivePeers {
    pub(crate) fn new(network: NetworkHandle) -> Self {
        Self {
            inactivity_threshold: Duration::from_secs(60),
            network,
            interval: tokio::time::interval(Duration::from_secs(1)),
            last_activity: Default::default(),
            inactive_peers: Default::default(),
        }
    }

    pub(crate) fn track_activity(&mut self, peer_id: PeerId) {
        if let Some(instant) = self.last_activity.get_mut(&peer_id) {
            *instant = Instant::now();
        }
    }

    pub(crate) fn start_tracking(&mut self, peer_id: PeerId) {
        self.last_activity.insert(peer_id, Instant::now());
    }

    pub(crate) fn stop_tracking(&mut self, peer_id: PeerId) {
        self.last_activity.remove(&peer_id);
    }
}

impl Stream for InactivePeers {
    type Item = PeerId;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {

        if self.network.is_syncing() {
            return Poll::Pending;
        }

        let mut this = self.project();

        // Drain the queue of inactive peers
        if let Some(peer) = this.inactive_peers.pop_front() {
            return Poll::Ready(Some(peer))
        }

        if this.interval.poll_tick(cx).is_ready() {
            for (peer_id, time) in this.last_activity.iter() {
                if time.elapsed().ge(this.inactivity_threshold) {
                    this.inactive_peers.push_back(*peer_id);
                }
            }
        }

        if !this.inactive_peers.is_empty() {
            cx.waker().wake_by_ref();
        }

        Poll::Pending
    }
}
