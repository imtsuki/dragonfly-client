/*
 *     Copyright 2024 The Dragonfly Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use dragonfly_client_core::{
    error::{ErrorType, OrErr},
    Result,
};
use sha2::{Digest, Sha256};
use url::Url;
use uuid::Uuid;

// IDGenerator is used to generate the id for the resources.
#[derive(Debug)]
pub struct IDGenerator {
    // ip is the ip of the host.
    ip: String,

    // hostname is the hostname of the host.
    hostname: String,

    // is_seed_peer indicates whether the host is a seed peer.
    is_seed_peer: bool,
}

// IDGenerator implements the IDGenerator.
impl IDGenerator {
    // new creates a new IDGenerator.
    pub fn new(ip: String, hostname: String, is_seed_peer: bool) -> Self {
        IDGenerator {
            ip,
            hostname,
            is_seed_peer,
        }
    }

    // host_id generates the host id.
    pub fn host_id(&self) -> String {
        if self.is_seed_peer {
            return format!("{}-{}-{}", self.ip, self.hostname, "seed");
        }

        format!("{}-{}", self.ip, self.hostname)
    }

    // task_id generates the task id.
    pub fn task_id(
        &self,
        url: &str,
        digest: Option<&str>,
        tag: Option<&str>,
        application: Option<&str>,
        piece_length: u64,
        filtered_query_params: Vec<String>,
    ) -> Result<String> {
        // Filter the query parameters.
        let url = Url::parse(url).or_err(ErrorType::ParseError)?;
        let query = url
            .query_pairs()
            .filter(|(k, _)| filtered_query_params.contains(&k.to_string()));
        let mut artifact_url = url.clone();
        artifact_url.query_pairs_mut().clear().extend_pairs(query);

        // Initialize the hasher.
        let mut hasher = Sha256::new();

        // Add the url to generate the task id.
        hasher.update(artifact_url.to_string());

        // Add the digest to generate the task id.
        if let Some(digest) = digest {
            hasher.update(digest);
        }

        // Add the tag to generate the task id.
        if let Some(tag) = tag {
            hasher.update(tag);
        }

        // Add the application to generate the task id.
        if let Some(application) = application {
            hasher.update(application);
        }

        // Add the piece length to generate the task id.
        hasher.update(piece_length.to_string());

        // Generate the task id.
        Ok(hex::encode(hasher.finalize()))
    }

    // peer_id generates the peer id.
    pub fn peer_id(&self) -> String {
        if self.is_seed_peer {
            return format!(
                "{}-{}-{}-{}",
                self.ip,
                self.hostname,
                Uuid::new_v4(),
                "seed",
            );
        }

        format!("{}-{}-{}", self.ip, self.hostname, Uuid::new_v4())
    }
}
