use super::Query;
use crate::error::Result;
/// 删除评论
/// 对应 Node.js module/comment_delete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

/// threadId = 资源类型前缀 + id（type 缺省为 0 歌曲）
pub(super) fn thread_id(query: &Query) -> String {
    let prefix = crate::util::config::RESOURCE_TYPE_MAP
        .get(query.get_or("type", "0").as_str())
        .copied()
        .unwrap_or("R_SO_4_");
    format!("{}{}", prefix, query.get_or("id", ""))
}

impl ApiClient {
    /// 删除评论
    /// 对应 /comment/delete
    pub async fn comment_delete(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "commentId": query.get_or("commentId", ""),
            "threadId": thread_id(query)
        });
        self.request(
            "/api/resource/comments/delete",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
