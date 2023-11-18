pub struct Records {
    pub records: Vec<String>,
}

impl Default for Records {
    fn default() -> Self {
        let records = [
            "url",
            "name",
            "mail",
            "email",
            "avatar",
            "header",
            "display",
            "location",
            "timezone",
            "language",
            "pronouns",
            "com.github",
            "org.matrix",
            "io.keybase",
            "description",
            "com.twitter",
            "com.discord",
            "social.bsky",
            "org.telegram",
            "social.mastodon",
            "network.dm3.profile",
            "network.dm3.deliveryService",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        Self { records }
    }
}
