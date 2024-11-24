use github_v3::*;
use std::sync::LazyLock;

/// A request builder for github apis.
pub fn github() -> Builder {
    static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        let token = std::env::var("GH_TOKEN")
            .or_else(|_| std::env::var("GITHUB_TOKEN"))
            .ok();
        Client::new(token.as_deref())
    });
    CLIENT.get()
}
