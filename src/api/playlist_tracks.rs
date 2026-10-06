use super::Query;
use crate::error::{NcmError, Result};
/// 收藏/取消收藏歌曲到歌单
/// 对应 Node.js module/playlist_tracks.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/删除歌曲到歌单
    /// 对应 /playlist/tracks
    ///
    /// 走 weapi：eapi 通道下该端点在部分网络环境会直接发送失败（issue #2）
    pub async fn playlist_tracks(&self, query: &Query) -> Result<ApiResponse> {
        let tracks: Vec<String> = query
            .get_or("tracks", "")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let build = |ids: &[String]| {
            json!({
                "op": query.get_or("op", "add"),
                "pid": query.get_or("pid", "0"),
                "trackIds": serde_json::to_string(ids).unwrap_or_default(),
                "imme": "true"
            })
        };
        let res = self
            .request(
                "/api/playlist/manipulate/tracks",
                build(&tracks),
                query.to_option(CryptoType::Weapi),
            )
            .await;
        match res {
            // 512 时与 Node 版一致：重复一遍 trackIds 后重试
            Err(NcmError::Api { code: 512, .. }) => {
                let doubled = [tracks.clone(), tracks].concat();
                self.request(
                    "/api/playlist/manipulate/tracks",
                    build(&doubled),
                    query.to_option(CryptoType::Weapi),
                )
                .await
            }
            other => other,
        }
    }
}
