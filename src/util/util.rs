use rand::{distributions::Alphanumeric, Rng};
pub fn random_token() {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(11) // PeerJS random token are 11 chars long
        .map(char::from)
        .collect();
}
