use {
    crate::{Error, Result},
    short_crypt::ShortCrypt,
    std::collections::HashMap,
};

/// Prefix used on the session cookie.
pub const PREFIX: &str = "__vial_";

pub const SECRET: &str = "__temp__";

/// A session store that can encrypt its values.
pub struct Session {
    store: HashMap<String, String>,
    secret: String,
}

impl Session {
    /// Create a new session store with a secret key.
    pub fn new(secret: &str) -> Session {
        Session {
            store: HashMap::new(),
            secret: secret.to_string(),
        }
    }

    /// Set a session value to the store.
    pub fn set(&mut self, name: &str, val: &str) {
        self.store
            .insert(name.to_lowercase().to_owned(), val.to_owned());
    }

    /// Get a session value from the store.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.store.get(&name.to_lowercase()).map(|v| v.as_ref())
    }

    /// Remove a session value from the store.
    pub fn remove(&mut self, name: &str) {
        self.store
            .insert(name.to_lowercase().to_owned(), "".to_owned());
    }

    /// Encode a value using the session secret.
    pub fn encode(&self, val: &str) -> String {
        let sc = ShortCrypt::new(&self.secret);
        sc.encrypt_to_url_component(val)
    }

    /// Decode a value using the session secret.
    pub fn decode(&self, val: &str) -> Result<String> {
        let sc = ShortCrypt::new(&self.secret);
        sc.decrypt_url_component(val)
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .map_err(|_| Error::SessionDecode)
    }
}
