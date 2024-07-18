
# tonic/tonic/Cargo.toml

    // As documented in 'tonic', the "transport" feature is defaulted but not required.
    // Curious that it doesn't seem to enable anything in the tonic/tonic/src directly.
    // You get the tonic::transport module regardless but with it, you do enable
    // two sub features, "server" and "channel".

    ./tonic/Cargo.toml:29:default = ["transport", "codegen", "prost"]


# tonic/tonic-reflection/Cargo.toml

    // Just a [dev-dependencies]
    // Enables the tonic "server" and "channel" features

    ./tonic-reflection/Cargo.toml:36:tonic = { version = "0.12", path = "../tonic", default-features = false, features = ["transport"] }

# tonic/tonic-health/Cargo.toml:18:default = ["transport"]

    // Curious how/why the health report can't provide 
    //    pub async fn set_serving<S>(&mut self)
    //    pub async fn set_not_serving<S>(&mut self)
    // when the "transport" feature isn't enabled.

    ./tonic-health/Cargo.toml:18:default = ["transport"]

    ./tonic-health/src/server.rs:11:#[cfg(feature = "transport")]
    ./tonic-health/src/server.rs:53:    #[cfg(feature = "transport")]
    ./tonic-health/src/server.rs:65:    #[cfg(feature = "transport")]

# tonic/tonic-build/Cargo.toml

    // Defines the fn generate_connect which returns a TokenStream

    ./tonic-build/Cargo.toml:25:default = ["transport", "prost"]

    ./tonic-build/src/client.rs:138:#[cfg(feature = "transport")]
    ./tonic-build/src/client.rs:161:#[cfg(not(feature = "transport"))]
