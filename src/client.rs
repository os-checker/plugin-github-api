use github_v3::*;
use std::sync::LazyLock;

/// A request builder for github apis.
pub fn github() -> Builder {
    static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new_from_env());
    CLIENT.get()
}
