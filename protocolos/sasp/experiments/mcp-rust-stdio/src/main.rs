use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
    handler::server::wrapper::{Json, Parameters},
    model::{
        CallToolResult, ContentBlock, ListResourcesResult, PaginatedRequestParams,
        ReadResourceRequestParams, ReadResourceResponse, ReadResourceResult, Resource,
        ResourceContents, ServerCapabilities, ServerConfig,
    },
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

const MUSIC_MANIFEST: &str =
    include_str!("../../../examples/music-player/manifest.json");
const BACKUP_MANIFEST: &str =
    include_str!("../../../examples/backup-daemon/manifest.json");
const BACKUP_STATE: &str =
    include_str!("../../../examples/backup-daemon/state.snapshot.json");

const MUSIC_MANIFEST_URI: &str = "sasp://music/manifest";
const MUSIC_STATE_URI: &str = "sasp://music/instances/music-01/state";
const BACKUP_MANIFEST_URI: &str = "sasp://backup/manifest";
const BACKUP_STATE_URI: &str = "sasp://backup/instances/backup-01/state";

#[derive(Debug, Clone)]
struct MusicState {
    playback: String,
    track: String,
    volume: u8,
    revision: u64,
}

impl Default for MusicState {
    fn default() -> Self {
        Self {
            playback: "paused".to_owned(),
            track: "Demo Track".to_owned(),
            volume: 45,
            revision: 7,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetVolume {
    #[schemars(range(min = 0, max = 100))]
    volume: u8,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct VerifyBackup {
    #[schemars(length(min = 1))]
    backup_id: String,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Accepted {
    accepted: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Verification {
    verified: bool,
}

#[derive(Clone)]
struct SaspDemo {
    music: Arc<Mutex<MusicState>>,
    allow_demo_mutations: bool,
}

impl SaspDemo {
    fn new(allow_demo_mutations: bool) -> Self {
        Self {
            music: Arc::new(Mutex::new(MusicState::default())),
            allow_demo_mutations,
        }
    }

    fn mutation_denied() -> CallToolResult {
        CallToolResult::error(vec![ContentBlock::text(
            "Denied by demo policy: set SASP_DEMO_ALLOW_MUTATIONS=1 to allow synthetic changes",
        )])
    }

    async fn music_snapshot(&self) -> String {
        let music = self.music.lock().await;
        json!({
            "kind": "state.snapshot",
            "provider_id": "org.example.music",
            "instance_id": "music-01",
            "revision": music.revision,
            "state": {
                "playback": music.playback,
                "track": music.track,
                "volume": music.volume
            }
        })
        .to_string()
    }
}

#[tool_router]
impl SaspDemo {
    #[tool(name = "player.play", description = "Resume synthetic music playback")]
    async fn player_play(&self) -> Result<Json<Accepted>, CallToolResult> {
        if !self.allow_demo_mutations {
            return Err(Self::mutation_denied());
        }
        let mut music = self.music.lock().await;
        if music.playback != "playing" {
            music.playback = "playing".to_owned();
            music.revision += 1;
        }
        Ok(Json(Accepted { accepted: true }))
    }

    #[tool(name = "player.set_volume", description = "Set synthetic player volume from 0 to 100")]
    async fn player_set_volume(
        &self,
        Parameters(SetVolume { volume }): Parameters<SetVolume>,
    ) -> Result<Json<Accepted>, CallToolResult> {
        if volume > 100 {
            return Err(CallToolResult::error(vec![ContentBlock::text(
                "Volume must be between 0 and 100",
            )]));
        }
        if !self.allow_demo_mutations {
            return Err(Self::mutation_denied());
        }
        let mut music = self.music.lock().await;
        if music.volume != volume {
            music.volume = volume;
            music.revision += 1;
        }
        Ok(Json(Accepted { accepted: true }))
    }

    #[tool(name = "backup.verify", description = "Verify a synthetic backup identifier")]
    fn backup_verify(
        &self,
        Parameters(VerifyBackup { backup_id }): Parameters<VerifyBackup>,
    ) -> Json<Verification> {
        // No filesystem access. Only the fixed synthetic identifier is considered valid.
        Json(Verification {
            verified: backup_id == "demo-backup-001",
        })
    }
}

#[tool_handler]
impl ServerHandler for SaspDemo {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                Resource::new(MUSIC_MANIFEST_URI, "Music Player manifest"),
                Resource::new(MUSIC_STATE_URI, "Music Player state"),
                Resource::new(BACKUP_MANIFEST_URI, "Backup Daemon manifest"),
                Resource::new(BACKUP_STATE_URI, "Backup Daemon state"),
            ],
            ..Default::default()
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        let uri = request.uri;
        let text = match uri.as_str() {
            MUSIC_MANIFEST_URI => MUSIC_MANIFEST.to_owned(),
            MUSIC_STATE_URI => self.music_snapshot().await,
            BACKUP_MANIFEST_URI => BACKUP_MANIFEST.to_owned(),
            BACKUP_STATE_URI => BACKUP_STATE.to_owned(),
            _ => return Err(McpError::resource_not_found(
                "Unknown resource",
                Some(json!({ "uri": uri })),
            )),
        };
        Ok(ReadResourceResult::new(vec![ResourceContents::text(text, uri)]).into())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This toggle is an experimental switch, NOT authentication or a production policy.
    let allow_demo_mutations =
        std::env::var("SASP_DEMO_ALLOW_MUTATIONS").ok().as_deref() == Some("1");
    let service = SaspDemo::new(allow_demo_mutations).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mutations_are_denied_by_default() {
        let server = SaspDemo::new(false);
        let result = server
            .player_set_volume(Parameters(SetVolume { volume: 70 }))
            .await;
        assert!(result.is_err());
        let snapshot: serde_json::Value =
            serde_json::from_str(&server.music_snapshot().await).unwrap();
        assert_eq!(snapshot["state"]["volume"], 45);
        assert_eq!(snapshot["revision"], 7);
    }

    #[tokio::test]
    async fn allowed_mutation_updates_state_revision() {
        let server = SaspDemo::new(true);
        let result = server
            .player_set_volume(Parameters(SetVolume { volume: 70 }))
            .await;
        assert!(result.is_ok());
        let snapshot: serde_json::Value =
            serde_json::from_str(&server.music_snapshot().await).unwrap();
        assert_eq!(snapshot["state"]["volume"], 70);
        assert_eq!(snapshot["revision"], 8);
    }

    #[test]
    fn manifests_are_parseable_and_identify_distinct_providers() {
        let music: serde_json::Value = serde_json::from_str(MUSIC_MANIFEST).unwrap();
        let backup: serde_json::Value = serde_json::from_str(BACKUP_MANIFEST).unwrap();
        assert_eq!(music["provider"]["id"], "org.example.music");
        assert_eq!(backup["provider"]["id"], "org.example.backup");
        assert_eq!(music["interaction"]["title"], "Music Player");
        assert!(backup.get("interaction").is_none());
    }
}
