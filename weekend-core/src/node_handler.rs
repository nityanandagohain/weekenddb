use crate::server::ServerThread;
use crate::hash_ring::HashRing;
use crate::kv_store::KvStore;
use crate::lattices::lww_lattice::{LWWLattice, TimestampValuePair};
use crate::zmq::ZMQSocketCache;

pub fn join_node(curr_thread: ServerThread,
                 new_thread: ServerThread,
                 global_hash_ring: &mut HashRing,
                 local_hash_ring: &HashRing,
                 kv_store: &KvStore<String, LWWLattice<Vec<u8>>, TimestampValuePair<Vec<u8>>>,
                 socket_cache: &mut ZMQSocketCache,
                 new_join_count: usize,
                 curr_join_count: usize) {
    let inserted = global_hash_ring.insert(new_thread, curr_join_count);

    if inserted {
        if curr_thread.thread_id == 0 {
            let socket = socket_cache.get_or_connect(new_thread.get_id());
            socket.send_string(curr_thread.get_id().as_str());
        }
    }
}