//! Search API types and functions

use serde::Deserialize;

/// Bilibili search type enum matching the API's search_type parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchType {
    Video,
    MediaBangumi,
    MediaFt,
    LiveRoom,
    LiveUser,
    User,
    Article,
    Topic,
}

impl SearchType {
    pub fn all() -> &'static [SearchType] {
        &[
            SearchType::Video,
            SearchType::MediaBangumi,
            SearchType::MediaFt,
            SearchType::LiveRoom,
            SearchType::LiveUser,
            SearchType::User,
            SearchType::Article,
            SearchType::Topic,
        ]
    }

    pub fn api_value(&self) -> &'static str {
        match self {
            SearchType::Video => "video",
            SearchType::MediaBangumi => "media_bangumi",
            SearchType::MediaFt => "media_ft",
            SearchType::LiveRoom => "live_room",
            SearchType::LiveUser => "live_user",
            SearchType::User => "user",
            SearchType::Article => "article",
            SearchType::Topic => "topic",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SearchType::Video => "视频",
            SearchType::MediaBangumi => "番剧",
            SearchType::MediaFt => "影视",
            SearchType::LiveRoom => "直播",
            SearchType::LiveUser => "主播",
            SearchType::User => "用户",
            SearchType::Article => "专栏",
            SearchType::Topic => "话题",
        }
    }
}

/// Sort order for search results (maps to the API's `order` parameter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchOrder {
    /// Comprehensive ranking (default).
    Totalrank,
    /// Most played.
    Click,
    /// Newest published.
    Pubdate,
    /// Most danmaku.
    Danmaku,
    /// Most favorited.
    Stow,
    /// Most comments.
    Scores,
}

impl SearchOrder {
    pub const ALL: [Self; 6] = [
        Self::Totalrank,
        Self::Click,
        Self::Pubdate,
        Self::Danmaku,
        Self::Stow,
        Self::Scores,
    ];

    pub fn api_value(self) -> &'static str {
        match self {
            Self::Totalrank => "totalrank",
            Self::Click => "click",
            Self::Pubdate => "pubdate",
            Self::Danmaku => "dm",
            Self::Stow => "stow",
            Self::Scores => "scores",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Totalrank => "综合",
            Self::Click => "播放",
            Self::Pubdate => "最新",
            Self::Danmaku => "弹幕",
            Self::Stow => "收藏",
            Self::Scores => "评论",
        }
    }

    pub fn cycle(self, direction: i32) -> Self {
        let index = Self::ALL
            .iter()
            .position(|order| *order == self)
            .unwrap_or(0);
        if direction >= 0 {
            Self::ALL[(index + 1) % Self::ALL.len()]
        } else {
            Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
        }
    }
}

/// Search result for video type
#[derive(Debug, Deserialize)]
pub struct SearchData {
    pub result: Option<Vec<SearchVideoItem>>,
    #[serde(rename = "numResults")]
    pub num_results: Option<i32>,
    pub page: Option<i32>,
    pub pagesize: Option<i32>,
}

/// Individual video search result
#[derive(Debug, Clone, Deserialize)]
pub struct SearchVideoItem {
    pub aid: Option<i64>,
    pub bvid: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub pic: Option<String>,
    pub play: Option<i64>,
    pub duration: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "danmaku")]
    pub danmaku: Option<i64>,
    pub mid: Option<i64>,
    #[serde(default)]
    pub pubdate: Option<i64>,
}

impl SearchVideoItem {
    pub fn display_title(&self) -> String {
        self.title
            .as_deref()
            .unwrap_or("无标题")
            .replace("<em class=\"keyword\">", "")
            .replace("</em>", "")
    }

    pub fn author_name(&self) -> &str {
        self.author.as_deref().unwrap_or("未知")
    }

    pub fn format_play(&self) -> String {
        match self.play {
            Some(n) if n >= 10000 => format!("{:.1}万", n as f64 / 10000.0),
            Some(n) => format!("{}", n),
            None => "-".to_string(),
        }
    }

    pub fn cover_url(&self) -> Option<String> {
        self.pic.as_ref().map(|url| {
            if url.starts_with("//") {
                format!("https:{}", url)
            } else {
                url.clone()
            }
        })
    }
}

/// Hot search item from web endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct HotwordItem {
    pub keyword: Option<String>,
    pub show_name: Option<String>,
    pub icon: Option<String>,
    pub pos: Option<i32>,
    pub word_type: Option<i32>,
}

impl HotwordItem {
    /// Display name prefers show_name over keyword
    pub fn display_text(&self) -> String {
        self.show_name
            .as_ref()
            .or(self.keyword.as_ref())
            .cloned()
            .unwrap_or_else(|| "-".to_string())
    }

    /// Keyword to trigger search
    pub fn keyword_text(&self) -> Option<String> {
        self.keyword
            .clone()
            .or_else(|| self.show_name.clone())
            .filter(|s| !s.is_empty())
    }

    /// Optional badge based on word_type
    pub fn badge(&self) -> Option<&'static str> {
        match self.word_type.unwrap_or_default() {
            4 => Some("新"),
            5 => Some("热"),
            7 => Some("直播"),
            9 => Some("梗"),
            11 => Some("话题"),
            12 => Some("独家"),
            _ => None,
        }
    }
}

/// Response for hot search list (web)
#[derive(Debug, Deserialize)]
pub struct HotwordResponse {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub list: Option<Vec<HotwordItem>>, // Top 10 hot words
}

#[cfg(test)]
mod tests {
    use super::SearchOrder;

    #[test]
    fn search_order_maps_to_api_values() {
        assert_eq!(SearchOrder::Totalrank.api_value(), "totalrank");
        assert_eq!(SearchOrder::Click.api_value(), "click");
        assert_eq!(SearchOrder::Pubdate.api_value(), "pubdate");
        assert_eq!(SearchOrder::Danmaku.api_value(), "dm");
        assert_eq!(SearchOrder::Stow.api_value(), "stow");
        assert_eq!(SearchOrder::Scores.api_value(), "scores");
    }

    #[test]
    fn search_order_cycles_wrap_around() {
        assert_eq!(SearchOrder::Totalrank.cycle(1), SearchOrder::Click);
        assert_eq!(SearchOrder::Totalrank.cycle(-1), SearchOrder::Scores);

        let mut order = SearchOrder::Totalrank;
        for _ in 0..SearchOrder::ALL.len() {
            order = order.cycle(1);
        }
        assert_eq!(order, SearchOrder::Totalrank);
    }
}
