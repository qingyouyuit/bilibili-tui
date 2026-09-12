use super::video_card::{VideoCard, VideoCardGrid};
use super::{Component, Theme, format_pubdate};
use crate::api::video::UpVideoItem;
use crate::application::AppAction;
use crate::storage::Keybindings;
use ratatui::{
    crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
    prelude::*,
    widgets::*,
};
use std::time::Instant;

pub struct UpVideoListPage {
    pub mid: i64,
    pub name: String,
    pub videos: Vec<UpVideoItem>,
    pub grid: VideoCardGrid,
    pub loading: bool,
    pub loading_more: bool,
    pub has_more: bool,
    pub page: i32,
    pub error_message: Option<String>,
    last_click_time: Option<Instant>,
    last_click_index: Option<usize>,
}

impl UpVideoListPage {
    pub fn new(mid: i64, name: String) -> Self {
        let mut grid = VideoCardGrid::new();
        grid.columns = 3;
        grid.card_height = 8;

        Self {
            mid,
            name,
            videos: Vec::new(),
            grid,
            loading: true,
            loading_more: false,
            has_more: true,
            page: 1,
            error_message: None,
            last_click_time: None,
            last_click_index: None,
        }
    }

    pub fn poll_cover_results(&mut self) {
        self.grid.poll_cover_results();
    }

    pub fn start_cover_downloads(&mut self) {
        self.grid.start_cover_downloads();
    }

    pub fn set_videos(
        &mut self,
        videos: Vec<UpVideoItem>,
        name: String,
        has_more: bool,
        page: i32,
    ) {
        self.name = name;
        self.videos = videos.clone();
        self.page = page;
        self.has_more = has_more;
        self.loading = false;
        self.error_message = None;
        self.grid.clear();
        for v in &videos {
            let card = VideoCard::new(
                Some(v.bvid.clone()),
                Some(v.aid),
                v.title.clone(),
                v.author.clone(),
                v.format_views(),
                v.format_duration(),
                Some(v.cover_url()),
            )
            .with_pubdate(Some(format_pubdate(v.created)));
            self.grid.add_card(card);
        }
    }

    pub fn append_videos(&mut self, videos: Vec<UpVideoItem>, has_more: bool, page: i32) {
        self.page = page;
        self.has_more = has_more;
        self.loading_more = false;
        self.videos.extend(videos.clone());
        for v in &videos {
            let card = VideoCard::new(
                Some(v.bvid.clone()),
                Some(v.aid),
                v.title.clone(),
                v.author.clone(),
                v.format_views(),
                v.format_duration(),
                Some(v.cover_url()),
            )
            .with_pubdate(Some(format_pubdate(v.created)));
            self.grid.add_card(card);
        }
    }

    pub fn begin_load_more(&mut self) -> bool {
        if !self.has_more || self.loading_more {
            return false;
        }
        self.loading_more = true;
        true
    }
}

impl Component for UpVideoListPage {
    fn draw(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, keys: &Keybindings) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(8),    // Video grid
                Constraint::Length(2), // Help
            ])
            .split(area);

        // Header
        let header = Paragraph::new(Line::from(vec![
            Span::styled("👤 ", Style::default().fg(theme.bilibili_pink)),
            Span::styled(
                format!("UP主: {}", self.name),
                Style::default()
                    .fg(theme.bilibili_pink)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" (共 {} 个视频)", self.videos.len()),
                Style::default().fg(theme.fg_secondary),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border_subtle)),
        );
        frame.render_widget(header, chunks[0]);

        if self.loading {
            let loading = Paragraph::new("⏳ 加载中...")
                .style(Style::default().fg(theme.warning))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(loading, chunks[1]);
        } else if let Some(error) = &self.error_message {
            let error_widget = Paragraph::new(format!("❌ {}", error))
                .style(Style::default().fg(theme.error))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(error_widget, chunks[1]);
        } else {
            self.grid.render(frame, chunks[1], theme);
        }

        // Help
        let help_text = if self.loading {
            String::new()
        } else {
            format!(
                "[{}/{}] 选择  [{}] 播放/进入  [{}] 加载更多  [{}] 返回",
                keys.nav_up, keys.nav_down, keys.confirm, keys.refresh, keys.back
            )
        };
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(theme.fg_secondary))
            .alignment(Alignment::Center);
        frame.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, key: KeyCode, keys: &Keybindings) -> Option<AppAction> {
        if keys.matches_quit(key) || keys.matches_back(key) {
            return Some(AppAction::BackToList);
        }
        if keys.matches_up(key) && self.grid.move_up() {
            return Some(AppAction::None);
        }
        if keys.matches_down(key) || key == KeyCode::Down {
            if self.grid.move_down() {
                if self.grid.is_near_bottom(2) && self.has_more && !self.loading_more {
                    return Some(AppAction::LoadMoreUpVideoList);
                }
                return Some(AppAction::None);
            }
            if self.has_more && !self.loading_more {
                return Some(AppAction::LoadMoreUpVideoList);
            }
            return Some(AppAction::None);
        }
        if keys.matches_left(key) {
            self.grid.move_left();
            return Some(AppAction::None);
        }
        if keys.matches_right(key) {
            self.grid.move_right();
            return Some(AppAction::None);
        }
        if keys.matches_refresh(key) {
            if self.has_more && !self.loading_more {
                return Some(AppAction::LoadMoreUpVideoList);
            }
            return Some(AppAction::None);
        }
        if keys.matches_confirm(key)
            && let Some(card) = self.grid.selected_card()
            && let Some(bvid) = &card.bvid
        {
            let aid = card.aid.unwrap_or(0);
            return Some(AppAction::OpenVideoDetail(bvid.clone(), aid));
        }
        Some(AppAction::None)
    }

    fn handle_mouse(&mut self, event: MouseEvent, area: Rect) -> Option<AppAction> {
        match event.kind {
            MouseEventKind::ScrollDown => {
                if self.grid.move_down()
                    && self.grid.is_near_bottom(2)
                    && self.has_more
                    && !self.loading_more
                {
                    return Some(AppAction::LoadMoreUpVideoList);
                }
                None
            }
            MouseEventKind::ScrollUp => {
                self.grid.move_up();
                None
            }
            MouseEventKind::Down(MouseButton::Left) => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(8),
                        Constraint::Length(2),
                    ])
                    .split(area);

                if self.loading {
                    return None;
                }

                let grid_area = chunks[1];

                if !grid_area.contains(ratatui::layout::Position::new(event.column, event.row)) {
                    return None;
                }

                let relative_y = event.row - grid_area.y;
                let click_row = (relative_y / self.grid.card_height) as usize;
                let actual_row = self.grid.scroll_row + click_row;

                let card_width = grid_area.width / self.grid.columns as u16;
                let click_col = (event.column.saturating_sub(grid_area.x) / card_width) as usize;

                let click_idx = actual_row * self.grid.columns + click_col;

                if click_idx < self.grid.cards.len() {
                    let now = Instant::now();
                    let is_double_click = self.last_click_index == Some(click_idx)
                        && self
                            .last_click_time
                            .is_some_and(|t| now.duration_since(t).as_millis() < 500);

                    if is_double_click {
                        self.last_click_time = None;
                        self.last_click_index = None;
                        if let Some(card) = self.grid.cards.get(click_idx)
                            && let Some(ref bvid) = card.bvid
                        {
                            let aid = card.aid.unwrap_or(0);
                            return Some(AppAction::OpenVideoDetail(bvid.clone(), aid));
                        }
                    } else {
                        self.grid.selected_index = click_idx;
                        self.grid.update_scroll(self.grid.cached_visible_rows);
                        self.last_click_time = Some(now);
                        self.last_click_index = Some(click_idx);
                    }
                }
                None
            }
            _ => None,
        }
    }
}
