use crate::api::search::SearchType;
use crate::app::{App, PreviousPage};
use crate::application::{AppAction, network};
use crate::infrastructure::{media, persistence};
use crate::presentation::tui::{
    BangumiDetailPage, BangumiPage, DynamicDetailPage, DynamicPage, HistoryPage, HomePage,
    LiveDetailPage, LivePage, LoginPage, NavItem, Page, SearchPage, SettingsPage, Theme,
    UpVideoListPage,
};

impl App {
    fn login_required_message() -> String {
        "该功能需要登录，请前往设置页登录".to_string()
    }

    fn apply_login_required_hint(&mut self) {
        let msg = Self::login_required_message();
        match &mut self.current_page {
            Page::Dynamic(page) => {
                page.loading_up_list = false;
                page.set_error(msg);
            }
            Page::History(page) => page.apply_load_more_error(msg),
            Page::VideoDetail(page) => {
                page.error_message = Some(msg);
                page.loading = false;
            }
            Page::DynamicDetail(page) => {
                page.error_message = Some(msg);
                page.loading = false;
            }
            Page::UpVideoList(page) => {
                page.error_message = Some(msg);
                page.loading = false;
            }
            _ => {}
        }
    }

    /// 记录当前页面以便返回导航
    fn save_previous_page(&mut self) {
        self.previous_page = match &self.current_page {
            Page::Home(_) => Some(PreviousPage::Home),
            Page::Search(_) => Some(PreviousPage::Search),
            Page::Dynamic(_) => Some(PreviousPage::Dynamic),
            Page::History(_) => Some(PreviousPage::History),
            Page::Live(_) => Some(PreviousPage::Live),
            Page::Bangumi(_) => Some(PreviousPage::Bangumi),
            Page::VideoDetail(page) => Some(PreviousPage::VideoDetail {
                bvid: page.bvid.clone(),
                aid: page.aid,
            }),
            _ => None,
        };
    }

    /// 缓存当前列表页状态，以便返回时保留查询结果/滚动位置
    fn cache_current_page(&mut self) {
        let placeholder = Page::Home(HomePage::new());
        let old = std::mem::replace(&mut self.current_page, placeholder);
        match old {
            Page::Home(page) => self.cached_home = Some(page),
            Page::Search(page) => self.cached_search = Some(page),
            Page::Dynamic(page) => self.cached_dynamic = Some(page),
            Page::History(page) => self.cached_history = Some(page),
            Page::Bangumi(page) => self.cached_bangumi = Some(*page),
            // 其他页面无法缓存，原样放回
            other => self.current_page = other,
        }
    }

    pub(super) async fn handle_action(&mut self, action: AppAction) {
        match action {
            AppAction::Quit => self.should_quit = true,
            AppAction::SwitchToHome => {
                self.sidebar.select(NavItem::Home);
                // Use cached home page if available
                if let Some(cached) = self.cached_home.take() {
                    self.current_page = Page::Home(cached);
                } else {
                    self.current_page = Page::Home(HomePage::new());
                    self.init_current_page().await;
                }
            }
            AppAction::RefreshHome => {
                self.sidebar.select(NavItem::Home);
                // Clear cache and create fresh home page
                self.cached_home = None;
                self.current_page = Page::Home(HomePage::new());
                self.init_current_page().await;
            }
            AppAction::SwitchToLogin => {
                self.current_page = Page::Login(LoginPage::new());
                self.init_current_page().await;
            }
            AppAction::LoginSuccess(creds) => {
                // Save credentials
                let _ = persistence::save_credentials(&creds);
                self.credentials = Some(creds.clone());
                // Update API client with new cookies
                {
                    let client = self.api_client.clone();
                    client.set_credentials(&creds);
                }
                // Switch to home
                self.current_page = Page::Home(HomePage::new());
                self.init_current_page().await;
            }
            AppAction::PlayVideo {
                bvid,
                aid,
                cid,
                duration,
            } => {
                let api_client = self.api_client.clone();
                let _ = media::play_video(
                    api_client,
                    &bvid,
                    aid,
                    cid,
                    duration,
                    None,
                    self.credentials.as_ref(),
                )
                .await;
            }
            AppAction::PlayVideoWithPages {
                bvid,
                aid,
                pages,
                current_index,
            } => {
                // Play only the selected episode
                if current_index < pages.len() {
                    let page = &pages[current_index];
                    let api_client = self.api_client.clone();
                    let _ = media::play_video(
                        api_client,
                        &bvid,
                        aid,
                        page.cid,
                        page.duration,
                        Some(page.page),
                        self.credentials.as_ref(),
                    )
                    .await;
                    // Update current page index in video detail page
                    if let Page::VideoDetail(detail_page) = &mut self.current_page
                        && detail_page.bvid == bvid
                    {
                        detail_page.current_page_index = current_index;
                    }
                }
            }
            AppAction::NavNext => {
                // Don't navigate if on video detail page
                if !matches!(self.current_page, Page::VideoDetail(_)) {
                    self.sidebar.next();
                    self.switch_to_nav_page().await;
                }
            }
            AppAction::NavPrev => {
                if !matches!(self.current_page, Page::VideoDetail(_)) {
                    self.sidebar.prev();
                    self.switch_to_nav_page().await;
                }
            }
            AppAction::Search(keyword) => {
                if let Page::Search(page) = &mut self.current_page {
                    page.query = keyword.clone();
                    page.page = 1;
                    page.loading = true;
                    page.show_hot_list = false;
                    let search_type = page.search_type;
                    let req_id = self.next_request_id("search");
                    if search_type == SearchType::Video {
                        self.send_network_command(network::NetworkCommand::Search {
                            req_id,
                            keyword,
                            page: 1,
                        });
                    } else {
                        self.send_network_command(network::NetworkCommand::SearchWithType {
                            req_id,
                            keyword,
                            page: 1,
                            search_type,
                        });
                    }
                }
            }
            AppAction::SearchWithType(keyword, st) => {
                if let Page::Search(page) = &mut self.current_page {
                    page.query = keyword.clone();
                    page.page = 1;
                    page.loading = true;
                    page.show_hot_list = false;
                    let req_id = self.next_request_id("search");
                    self.send_network_command(network::NetworkCommand::SearchWithType {
                        req_id,
                        keyword,
                        page: 1,
                        search_type: st,
                    });
                }
            }
            AppAction::SwitchSearchType(st) => {
                if let Page::Search(page) = &mut self.current_page {
                    if page.search_type != st {
                        page.switch_type(st);
                        if !page.query.is_empty() {
                            let keyword = page.query.clone();
                            let req_id = self.next_request_id("search");
                            self.send_network_command(network::NetworkCommand::SearchWithType {
                                req_id,
                                keyword,
                                page: 1,
                                search_type: st,
                            });
                        }
                    }
                }
            }
            AppAction::RefreshDynamic => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                if let Page::Dynamic(page) = &mut self.current_page {
                    page.loading = true;
                    let tab = page.current_tab;
                    let host_mid = page.get_selected_up_mid();
                    let req_id = self.next_request_id("dynamic_refresh");
                    self.send_network_command(network::NetworkCommand::LoadDynamicRefresh {
                        req_id,
                        tab,
                        host_mid,
                    });
                }
            }
            AppAction::OpenVideoDetail(bvid, aid) => {
                self.save_previous_page();
                // Cache current list page before navigating to video detail
                self.cache_current_page();
                let detail_page = crate::presentation::tui::VideoDetailPage::new(bvid.clone(), aid);
                self.current_page = Page::VideoDetail(Box::new(detail_page));
                let req_id = self.next_request_id("video_detail");
                self.send_network_command(network::NetworkCommand::LoadVideoDetail {
                    req_id,
                    bvid,
                    aid,
                });
            }
            AppAction::OpenDynamicDetail(dynamic_id) => {
                self.save_previous_page();
                // Cache current list page before navigating to dynamic detail
                self.cache_current_page();
                let detail_page = DynamicDetailPage::new(dynamic_id.clone());
                self.current_page = Page::DynamicDetail(Box::new(detail_page));
                let req_id = self.next_request_id("dynamic_detail");
                self.send_network_command(network::NetworkCommand::LoadDynamicDetail {
                    req_id,
                    dynamic_id,
                });
            }
            AppAction::BackToList => match self.previous_page.take() {
                Some(PreviousPage::Home) => {
                    self.sidebar.select(NavItem::Home);
                    // Use cached home page if available
                    if let Some(cached) = self.cached_home.take() {
                        self.current_page = Page::Home(cached);
                    } else {
                        self.current_page = Page::Home(HomePage::new());
                        self.init_current_page().await;
                    }
                }
                Some(PreviousPage::Search) => {
                    self.sidebar.select(NavItem::Search);
                    if let Some(cached) = self.cached_search.take() {
                        self.current_page = Page::Search(cached);
                    } else {
                        self.current_page = Page::Search(SearchPage::new());
                        self.init_current_page().await;
                    }
                }
                Some(PreviousPage::Dynamic) => {
                    self.sidebar.select(NavItem::Dynamic);
                    if let Some(cached) = self.cached_dynamic.take() {
                        self.current_page = Page::Dynamic(cached);
                    } else {
                        self.current_page = Page::Dynamic(DynamicPage::new());
                        self.init_current_page().await;
                    }
                }
                Some(PreviousPage::History) => {
                    self.sidebar.select(NavItem::History);
                    if let Some(cached) = self.cached_history.take() {
                        self.current_page = Page::History(cached);
                    } else {
                        self.current_page = Page::History(HistoryPage::new());
                        self.init_current_page().await;
                    }
                }
                Some(PreviousPage::Live) => {
                    self.sidebar.select(NavItem::Live);
                    self.current_page = Page::Live(LivePage::new());
                    self.init_current_page().await;
                }
                Some(PreviousPage::Bangumi) => {
                    self.sidebar.select(NavItem::Bangumi);
                    if let Some(cached) = self.cached_bangumi.take() {
                        self.current_page = Page::Bangumi(Box::new(cached));
                    } else {
                        self.current_page = Page::Bangumi(Box::<BangumiPage>::default());
                        self.init_current_page().await;
                    }
                }
                Some(PreviousPage::VideoDetail { bvid, aid }) => {
                    let detail_page = crate::presentation::tui::VideoDetailPage::new(bvid.clone(), aid);
                    self.current_page = Page::VideoDetail(Box::new(detail_page));
                    let req_id = self.next_request_id("video_detail");
                    self.send_network_command(network::NetworkCommand::LoadVideoDetail {
                        req_id,
                        bvid,
                        aid,
                    });
                }
                None => {
                    // Default to home
                    self.sidebar.select(NavItem::Home);
                    if let Some(cached) = self.cached_home.take() {
                        self.current_page = Page::Home(cached);
                    } else {
                        self.current_page = Page::Home(HomePage::new());
                        self.init_current_page().await;
                    }
                }
            },
            AppAction::LoadMoreRecommendations => {
                if let Page::Home(page) = &mut self.current_page
                    && let Some(fresh_idx) = page.begin_load_more()
                {
                    let req_id = self.next_request_id("home_more");
                    self.send_network_command(network::NetworkCommand::LoadHomeMore {
                        req_id,
                        fresh_idx,
                        use_guest_feed: self.credentials.is_none(),
                    });
                }
            }
            AppAction::LoadMoreSearch => {
                let mut command = None;
                if let Page::Search(page) = &mut self.current_page {
                    if page.loading_more || page.query.is_empty() || page.show_hot_list {
                        return;
                    }
                    if page.grid.cards.len() >= page.total_results as usize {
                        return;
                    }
                    page.loading_more = true;
                    let next_page = page.page + 1;
                    let search_type = page.search_type;
                    command = Some((page.query.clone(), next_page, search_type));
                }
                if let Some((keyword, next_page, search_type)) = command {
                    let req_id = self.next_request_id("search");
                    if search_type == SearchType::Video {
                        self.send_network_command(network::NetworkCommand::Search {
                            req_id,
                            keyword,
                            page: next_page,
                        });
                    } else {
                        self.send_network_command(network::NetworkCommand::SearchWithType {
                            req_id,
                            keyword,
                            page: next_page,
                            search_type,
                        });
                    }
                }
            }
            AppAction::LoadMoreDynamic => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                let mut command = None;
                if let Page::Dynamic(page) = &mut self.current_page {
                    if page.loading_more || !page.has_more {
                        return;
                    }
                    let Some(offset) = page.offset.clone() else {
                        return;
                    };
                    page.loading_more = true;
                    command = Some((offset, page.current_tab, page.get_selected_up_mid()));
                }
                if let Some((offset, tab, host_mid)) = command {
                    let req_id = self.next_request_id("dynamic_more");
                    self.send_network_command(network::NetworkCommand::LoadDynamicMore {
                        req_id,
                        offset,
                        tab,
                        host_mid,
                    });
                }
            }
            AppAction::LoadMoreHistory => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                if let Page::History(page) = &mut self.current_page
                    && let Some(cursor) = page.start_load_more_request()
                {
                    let req_id = self.next_request_id("history_more");
                    self.send_network_command(network::NetworkCommand::LoadHistoryMore {
                        req_id,
                        cursor,
                    });
                }
            }
            AppAction::SwitchToHistory => {
                self.sidebar.select(NavItem::History);
                self.current_page = Page::History(HistoryPage::new());
                self.init_current_page().await;
            }
            AppAction::LoadMoreComments => {
                if let Page::VideoDetail(page) = &mut self.current_page {
                    let client = self.api_client.clone();
                    page.load_more_comments(&client).await;
                } else if let Page::DynamicDetail(page) = &mut self.current_page {
                    let client = self.api_client.clone();
                    page.load_more_comments(&client).await;
                }
            }
            AppAction::ToggleCommentReplies => {
                if let Page::VideoDetail(page) = &mut self.current_page {
                    let client = self.api_client.clone();
                    page.toggle_comment_replies(&client).await;
                }
            }
            AppAction::SwitchDynamicTab(tab) => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                let mut command = None;
                if let Page::Dynamic(page) = &mut self.current_page {
                    page.switch_tab(tab);
                    let host_mid = page.get_selected_up_mid();
                    command = Some((page.current_tab, host_mid));
                }
                if let Some((tab, host_mid)) = command {
                    let req_id = self.next_request_id("dynamic_refresh");
                    self.send_network_command(network::NetworkCommand::LoadDynamicRefresh {
                        req_id,
                        tab,
                        host_mid,
                    });
                }
            }
            AppAction::SelectUpMaster(index) => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                let mut command = None;
                if let Page::Dynamic(page) = &mut self.current_page {
                    page.select_up(index);
                    let host_mid = page.get_selected_up_mid();
                    command = Some((page.current_tab, host_mid));
                }
                if let Some((tab, host_mid)) = command {
                    let req_id = self.next_request_id("dynamic_refresh");
                    self.send_network_command(network::NetworkCommand::LoadDynamicRefresh {
                        req_id,
                        tab,
                        host_mid,
                    });
                }
            }
            AppAction::NextTheme => {
                self.theme_id = Theme::next_theme_id(&self.theme_id);
                self.theme = Theme::load_or_default(&self.theme_id).0;
                self.save_theme_to_config();
            }
            AppAction::SetTheme(theme_id) => {
                self.theme_id = theme_id;
                self.theme = Theme::load_or_default(&self.theme_id).0;
                self.save_theme_to_config();
            }
            AppAction::SwitchToSettings => {
                self.sidebar.select(NavItem::Settings);
                let page = SettingsPage::new(
                    self.keybindings.clone(),
                    self.theme_id.clone(),
                    self.credentials.is_some(),
                );
                self.current_page = Page::Settings(Box::new(page));
            }
            AppAction::Logout => {
                let _ = persistence::delete_credentials();
                self.credentials = None;
                self.api_client.clear_credentials();
                self.cached_home = None;
                self.current_page = Page::Home(HomePage::new());
                self.init_current_page().await;
            }
            AppAction::LikeComment {
                oid,
                rpid,
                comment_type,
            } => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                let client = self.api_client.clone();
                // Toggle like - if already liked, unlike
                if let Page::VideoDetail(page) = &mut self.current_page {
                    let is_liked = page.liked_comments.contains(&rpid);
                    if let Ok(()) = client
                        .like_comment(oid, rpid, comment_type, !is_liked)
                        .await
                    {
                        if is_liked {
                            page.liked_comments.remove(&rpid);
                        } else {
                            page.liked_comments.insert(rpid);
                        }
                    }
                } else if let Page::DynamicDetail(page) = &mut self.current_page {
                    let is_liked = page.liked_comments.contains(&rpid);
                    if let Ok(()) = client
                        .like_comment(oid, rpid, comment_type, !is_liked)
                        .await
                    {
                        if is_liked {
                            page.liked_comments.remove(&rpid);
                        } else {
                            page.liked_comments.insert(rpid);
                        }
                    }
                }
            }
            AppAction::AddComment {
                oid,
                comment_type,
                message,
                root,
            } => {
                if self.credentials.is_none() {
                    self.apply_login_required_hint();
                    return;
                }
                let client = self.api_client.clone();
                if let Ok(_response) = client
                    .add_comment(oid, comment_type, &message, root, root)
                    .await
                {
                    // Reload comments to show new comment
                    if let Page::VideoDetail(page) = &mut self.current_page {
                        page.load_data(&client).await;
                    } else if let Page::DynamicDetail(page) = &mut self.current_page {
                        page.load_data(&client).await;
                    }
                }
            }
            AppAction::SaveKeybindings(new_keybindings) => {
                self.keybindings = (*new_keybindings).clone();
                self.config.keybindings = *new_keybindings;
                let _ = persistence::save_config(&self.config);
            }
            AppAction::SwitchToLive => {
                self.sidebar.select(NavItem::Live);
                self.current_page = Page::Live(LivePage::new());
                self.init_current_page().await;
            }
            AppAction::OpenLiveDetail(room_id) => {
                self.save_previous_page();
                let mut detail_page = LiveDetailPage::new(room_id);
                let client = &self.api_client;
                detail_page.load_room_info(client).await;
                // Connect WebSocket for real-time messages
                let uid = self
                    .credentials
                    .as_ref()
                    .and_then(|c| c.dede_user_id.parse::<i64>().ok())
                    .unwrap_or(0);
                detail_page.connect_ws(client, uid).await;
                self.current_page = Page::LiveDetail(Box::new(detail_page));
            }
            AppAction::RefreshLive => {
                if let Page::Live(page) = &mut self.current_page {
                    page.begin_loading();
                    let req_id = self.next_request_id("live_init");
                    self.send_network_command(network::NetworkCommand::LoadLiveInit { req_id });
                }
            }
            AppAction::LoadMoreLive => {
                if let Page::Live(page) = &mut self.current_page
                    && page.begin_load_more()
                {
                    let req_id = self.next_request_id("live_more");
                    self.send_network_command(network::NetworkCommand::LoadLiveMore { req_id });
                }
            }
            AppAction::PlayLive { room_id, title: _ } => {
                let _ = media::play_live(room_id).await;
            }
            AppAction::SwitchToBangumi => {
                self.sidebar.select(NavItem::Bangumi);
                if let Some(cached) = self.cached_bangumi.take() {
                    self.current_page = Page::Bangumi(Box::new(cached));
                } else {
                    self.current_page = Page::Bangumi(Box::<BangumiPage>::default());
                    self.init_current_page().await;
                }
            }
            AppAction::RefreshBangumi => {
                self.sidebar.select(NavItem::Bangumi);
                self.cached_bangumi = None;
                self.current_page = Page::Bangumi(Box::<BangumiPage>::default());
                self.init_current_page().await;
            }
            AppAction::SwitchBangumiTab(tab) => {
                if let Page::Bangumi(page) = &mut self.current_page {
                    page.switch_tab(tab);
                    let season_type = tab.season_type();
                    let req_id = self.next_request_id("bangumi_index");
                    self.send_network_command(network::NetworkCommand::LoadBangumiIndex {
                        req_id,
                        season_type,
                    });
                }
            }
            AppAction::OpenBangumiDetail(season_id) => {
                self.save_previous_page();
                if let Page::Bangumi(bangumi_page) = std::mem::replace(
                    &mut self.current_page,
                    Page::Bangumi(Box::<BangumiPage>::default()),
                ) {
                    self.cached_bangumi = Some(*bangumi_page);
                }
                let detail_page = BangumiDetailPage::new(season_id);
                self.current_page = Page::BangumiDetail(Box::new(detail_page));
                let req_id = self.next_request_id("bangumi_detail");
                self.send_network_command(network::NetworkCommand::LoadBangumiDetail {
                    req_id,
                    season_id,
                });
            }
            AppAction::LoadMoreBangumi => {
                // Rank API has no pagination
            }
            AppAction::PlayBangumiEpisode {
                ep_id,
                season_id: _,
                title: _,
            } => {
                let _ = media::play_bangumi_episode(ep_id, self.credentials.as_ref()).await;
            }
            AppAction::OpenUpVideoList { mid, name } => {
                self.save_previous_page();
                self.cache_current_page();
                let page = UpVideoListPage::new(mid, name.clone());
                self.current_page = Page::UpVideoList(Box::new(page));
                let req_id = self.next_request_id("up_videos");
                self.send_network_command(network::NetworkCommand::LoadUpVideos {
                    req_id,
                    mid,
                    page: 1,
                });
            }
            AppAction::LoadMoreUpVideos => {
                let mut command = None;
                if let Page::UpVideoList(page) = &mut self.current_page
                    && page.begin_load_more()
                {
                    let mid = page.mid;
                    let next_page = page.page + 1;
                    command = Some((mid, next_page));
                }
                if let Some((mid, next_page)) = command {
                    let req_id = self.next_request_id("up_videos");
                    self.send_network_command(network::NetworkCommand::LoadUpVideos {
                        req_id,
                        mid,
                        page: next_page,
                    });
                }
            }
            AppAction::None => {}
        }
    }

    async fn switch_to_nav_page(&mut self) {
        // Already on the target page type: keep the current state untouched
        let already_there = matches!(
            (&self.current_page, self.sidebar.selected),
            (Page::Home(_), NavItem::Home)
                | (Page::Search(_), NavItem::Search)
                | (Page::Dynamic(_), NavItem::Dynamic)
                | (Page::History(_), NavItem::History)
                | (Page::Live(_), NavItem::Live)
                | (Page::Bangumi(_), NavItem::Bangumi)
                | (Page::Settings(_), NavItem::Settings)
        );
        if already_there {
            return;
        }

        // Cache the page we're leaving (if cacheable) to preserve its state
        self.cache_current_page();

        match self.sidebar.selected {
            NavItem::Home => {
                if let Some(cached) = self.cached_home.take() {
                    self.current_page = Page::Home(cached);
                } else {
                    self.current_page = Page::Home(HomePage::new());
                    self.init_current_page().await;
                }
            }
            NavItem::Search => {
                if let Some(cached) = self.cached_search.take() {
                    self.current_page = Page::Search(cached);
                } else {
                    self.current_page = Page::Search(SearchPage::new());
                    self.init_current_page().await;
                }
            }
            NavItem::Dynamic => {
                if let Some(cached) = self.cached_dynamic.take() {
                    self.current_page = Page::Dynamic(cached);
                } else {
                    self.current_page = Page::Dynamic(DynamicPage::new());
                    self.init_current_page().await;
                }
            }
            NavItem::History => {
                if let Some(cached) = self.cached_history.take() {
                    self.current_page = Page::History(cached);
                } else {
                    self.current_page = Page::History(HistoryPage::new());
                    self.init_current_page().await;
                }
            }
            NavItem::Settings => {
                let page = SettingsPage::new(
                    self.keybindings.clone(),
                    self.theme_id.clone(),
                    self.credentials.is_some(),
                );
                self.current_page = Page::Settings(Box::new(page));
            }
            NavItem::Live => {
                self.current_page = Page::Live(LivePage::new());
                self.init_current_page().await;
            }
            NavItem::Bangumi => {
                if let Some(cached) = self.cached_bangumi.take() {
                    self.current_page = Page::Bangumi(Box::new(cached));
                } else {
                    self.current_page = Page::Bangumi(Box::<BangumiPage>::default());
                    self.init_current_page().await;
                }
            }
        }
    }

    pub(super) async fn init_current_page(&mut self) {
        match &mut self.current_page {
            Page::Login(page) => {
                let client = self.api_client.clone();
                page.load_qrcode(&client).await;
            }
            Page::Home(page) => {
                page.begin_loading();
                let req_id = self.next_request_id("home");
                self.send_network_command(network::NetworkCommand::LoadHome {
                    req_id,
                    use_guest_feed: self.credentials.is_none(),
                });
            }
            Page::Search(page) => {
                page.start_hotword_loading();
                let req_id = self.next_request_id("hotwords");
                self.send_network_command(network::NetworkCommand::LoadHotwords { req_id });
            }
            Page::Dynamic(page) => {
                if self.credentials.is_none() {
                    page.loading_up_list = false;
                    page.set_error(Self::login_required_message());
                    return;
                }
                page.loading_up_list = true;
                let tab = page.current_tab;
                let host_mid = page.get_selected_up_mid();
                let req_id = self.next_request_id("dynamic_init");
                self.send_network_command(network::NetworkCommand::LoadDynamicInit {
                    req_id,
                    tab,
                    host_mid,
                });
            }
            Page::VideoDetail(_) => {
                // VideoDetail is initialized when created
            }
            Page::DynamicDetail(_) => {
                // DynamicDetail is initialized when created
            }
            Page::History(page) => {
                if self.credentials.is_none() {
                    page.apply_load_more_error(Self::login_required_message());
                    return;
                }
                page.begin_loading();
                let req_id = self.next_request_id("history_init");
                self.send_network_command(network::NetworkCommand::LoadHistoryInit { req_id });
            }
            Page::Live(page) => {
                page.begin_loading();
                let req_id = self.next_request_id("live_init");
                self.send_network_command(network::NetworkCommand::LoadLiveInit { req_id });
            }
            Page::LiveDetail(page) => {
                let client = self.api_client.clone();
                page.load_room_info(&client).await;
            }
            Page::Settings(_) => {
                // Settings doesn't need async initialization
            }
            Page::Bangumi(page) => {
                page.loading = true;
                let season_type = page.current_tab.season_type();
                let req_id = self.next_request_id("bangumi_index");
                self.send_network_command(network::NetworkCommand::LoadBangumiIndex {
                    req_id,
                    season_type,
                });
            }
            Page::BangumiDetail(_) => {
                // BangumiDetail is initialized when created
            }
            Page::UpVideoList(_) => {
                // UpVideoList is initialized when created
            }
        }
    }

    fn save_theme_to_config(&mut self) {
        self.config.theme = self.theme_id.clone();
        if persistence::save_config(&self.config).is_err() {}
    }
}
