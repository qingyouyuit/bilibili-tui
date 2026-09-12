# Changelog

## [Unreleased]

### 🚀 Features

- **UP主空间页面**：在视频详情页按 `u` 键查看该 UP 主发布的所有视频列表
- **新增 `open_up` 键位绑定**（默认 `u`），可在设置页自定义
- 合并上游 v1.0.14：收藏夹、专栏阅读、CDN 优选、播放队列、自动连播、清晰度选择、弹幕配置等

### 🚜 Refactor

- 新增 API 端点 `get_up_videos()`，调用 `/x/space/wbi/arc/search`（WBI 签名)
- 新增 `NetworkCommand::LoadUpVideos` / `NetworkEvent::UpVideosLoaded`
- 新增 `AppAction::OpenUpVideoList` / `AppAction::LoadMoreUpVideos`
- 新增 `PreviousPage::VideoDetail` 变体，支持从 UP 主页面正确返回视频详情
- 新建 `UpVideoListPage` UI 组件，基于 `VideoCardGrid` 网格布局（3列）
- 新增 `get_with_wbi_json()` 方法用于返回原始 JSON 的 WBI 签名请求

### 🐛 Bug Fixes

- 修复 UP 主空间 API 反序列化失败：字段名 `length` 而非 `duration`，改用手动 JSON 解析
- 修复从搜索结果进入视频详情后按 Esc 丢失搜索结果的体验问题：新增搜索/动态/历史页面缓存，返回时恢复查询与滚动状态
- 修复搜索结果打开视频时评论加载错误：改用视频信息中的权威 `aid`，而非搜索项中的 UP 主 `mid`

---

## [1.0.14] - 2026-08-30

### 🚀 Features

- *(release)* Add mise task to release

### 🐛 Bug Fixes

- *(player)* Restore buffering for VOD playback (#41)
- *(home)* Allow typing 'i' in search input when focused (#39)
- *(settings)* Allow theme list to scroll beyond visible viewport (#38)

## [1.0.13] - 2026-08-03

### 🚀 Features

- *(login)* Add dual QR code rendering in login UI

## [1.0.12] - 2026-07-14

### 🚀 Features

- 添加 MPV Bilibili SponsorBlock 选项以跳过广告
- Add Bangumi functionality with timeline and ranking
- Improve browsing and resilient playback
- Persist CDN ranking metrics
- Rank regional CDN catalog
- *(player)* Optimize danmaku rendering and low-latency playback

### 🐛 Bug Fixes

- Harden playback and playlist lifecycle

### ⚙️ Miscellaneous Tasks

- *(release)* Bump version to 1.0.12 and update CHANGELOG

## [1.0.11] - 2026-04-20

### 🚀 Features

- 添加缓存可见行数以优化滚动和预加载逻辑

### 🐛 Bug Fixes

- 优化行加载逻辑以改善滚动体验
- Key binding conflicts in dynamic page

## [1.0.10] - 2026-04-07

### 🚀 Features

- Support guest mode without forced login

### 🐛 Bug Fixes

- *(history)* 支持直播历史回车进入直播详情

### 🚜 Refactor

- Decouple network worker from UI loop
- 重构工程分层并拆分 App 模块

### 📚 Documentation

- Update CHANGELOG.md for version 1.0.9

### ⚙️ Miscellaneous Tasks

- 更新依赖版本（anyhow， crossterm， reqwest， opaline， etc.）

## [1.0.9] - 2026-03-16

### 🚀 Features

- 直播弹幕 WebSocket 接入，支持实时弹幕、进场、礼物、高能榜消息
- 直播弹幕连接自动心跳保活（30s间隔），支持掉线自动重连
- 直播历史弹幕加载与展示
- 直播弹幕支持 brotli+zlib 解压
- *(bangumi)* 新增番剧索引列表，支持番剧/国创分类切换
- *(bangumi)* 番剧详情页，剧集列表分组展示(正片/OVA/预告等)
- *(bangumi)* 支持从番剧详情页播放番剧剧集

### 🐛 Bug Fixes

- *(bangumi)* 修复番剧页持久化反序列化兼容性
- *(bangumi)* 修复缓存清空逻辑
- 修复动态评论面板下方遮档内容
- 修复直播弹幕 zlib 解压的包体大小预处理
- 修复动态加载 up 列表失败导致流程卡死
- 修复直播推荐接口字段兼容性

### 🚜 Refactor

- 直播弹幕协议层抽象，cleanup websocket 包处理逻辑
- 重构 live 页面组件渲染逻辑
- 统一使用 `app_result` 作为 `Result` 别名
- 所有 API 错误统一通过 network worker 返回，UI 层只处理渲染
- 优化主题切换性能

### 📚 Documentation

- Add README section for Nix Flake support
- Update README with new feature preview (live, bangumi)
- 完善项目文档

## [1.0.8] - 2026-03-02

### 🚀 Features

- 支持简历/设置页面，可重新登录或退出登录
- 支持鼠标侧键前进后退（历史导航用）
- 动态页面支持加载更多（动态流无限滚动）
- 支持动态详情页面查看图片（画画/文字类动态）
- 支持在动态详情页面发布评论
- 支持在动态详情页面点赞评论

### 🐛 Bug Fixes

- 修复滚动时无法触发加载更多的问题
- 修复评论面板展开回复遮挡内容
- 修复空评论导致的 panic
- 修复 Esc 在动态页面不响应 back 操作

## [1.0.7] - 2026-02-15

### 🚀 Features

- 新增搜索页面支持 Bilibili 热搜词展示
- 搜索页面支持 8 种搜索类型（视频/番剧/影视/直播/用户/文章/合集）
- 支持番剧时间表/排行榜查看
- 视频详情页评论功能：发表评论、回复评论
- 视频详情页评论点赞/取消点赞
- 视频详情页选集列表（多 P 视频）
- 视频详情页切换焦点区域（Tab 切换评论/选集/相关推荐）
- 设置页面增加显示当前版本号和 commit hash
- 键盘快捷键支持设置页内直接编辑

### 🐛 Bug Fixes

- 修复翻页时搜索结果的加载状态错误
- 修复动态列表刷新时的闪烁问题
- 修复部分封面图无法正常渲染（协议头补全）
- 修复退出登录时未清理缓存的首页

## [1.0.6] - 2026-02-01

### 🚀 Features

- 支持视频播放心跳上报（开始播放、每15秒保活、播放结束）
- 支持多 P 视频自动连播下一集
- 新增键盘快捷键设置页面，支持全自定义键位绑定
- 设置页支持主题选择和实时预览
- 支持鼠标操作：点击选择、双击打开、滚轮滚动

### 🐛 Bug Fixes

- 修复部分场景下首页推荐加载失败
- 修复动态加载时 up 主列表为空导致的显示异常
- 修复 cookie 失效后未及时提示重新登录

## [1.0.5] - 2026-01-15

### 🚀 Features

- 动态页面改版，支持 up 主选择栏
- 动态页面支持全/all、视频/video、图文/image 三种分类 Tab
- 历史记录页面支持光标分页无限滚动
- 直播推荐页面（浏览、进入直播间）
- 支持退出登录（清除凭证）

### 🐛 Bug Fixes

- 修复首页推荐在未登录状态下的加载失败
- 修复搜索结果的封面图异步加载问题

## [1.0.4] - 2026-01-01

### 🚀 Features

- 视频搜索（WBI 签名）
- 视频详情页（标题、作者、统计信息、描述）
- 视频详情页相关推荐
- 视频详情页评论区（查看、展开回复）
- 视频播放（调用 mpv + yt-dlp）
- Cookie 导出供 yt-dlp 鉴权播放

### 🐛 Bug Fixes

- 修复部分 API 接口的字段兼容性问题

## [1.0.3] - 2025-12-20

### 🚀 Features

- 基于 Opaline 的 39 套内置主题（Catppuccin、Nord、Dracula、Tokyo Night等）
- 支持快捷键 `t` 切换主题
- 首屏推荐视频网格展示（登录用户个性化推荐/游客热门推荐）
- 封面图异步下载并渲染

### 🐛 Bug Fixes

- 修复配置文件加载逻辑

## [1.0.2] - 2025-12-10

### 🚀 Features

- 二维码登录（生成二维码 + 轮询扫码状态）
- 凭证持久化保存到本地文件
- 左侧导航栏（7 个导航项）
- 基础页面框架（Component trait、Page 枚举）

### 🐛 Bug Fixes

- 修复退出时终端未正确恢复

## [1.0.1] - 2025-12-01

### 🚀 Features

- 初始化项目结构
- Ratatui + crossterm 终端框架搭建
- 后台网络工作线程（独立 tokio runtime）
- 请求去重机制

### ⚙️ Miscellaneous Tasks

- 配置 CI/CD 自动构建
- 项目基础配置（选型、依赖管理）

---

# 项目概览

## 架构

```
src/
├── main.rs                   入口，初始化终端
├── lib.rs                    模块声明
├── api/                      Bilibili API 客户端
│   ├── client.rs             HTTP 客户端 (reqwest)、WBI 签名、Cookie 管理
│   ├── auth.rs               二维码登录类型
│   ├── recommend.rs          推荐数据类型
│   ├── video.rs              视频信息、相关视频、UP主空间类型
│   ├── search.rs             搜索类型 (8种) 和热词
│   ├── comment.rs            评论类型及 CRUD
│   ├── dynamic.rs            动态 feed、UP 列表、图文/opus 类型
│   ├── bangumi.rs            番剧时间线、排行榜、季度详情
│   ├── history.rs            历史记录 (cursor 分页)
│   ├── live.rs               直播推荐、房间信息
│   ├── live_ws.rs            WebSocket 弹幕协议 (brotli/zlib)
│   ├── live_client.rs        WebSocket 客户端
│   ├── heartbeat.rs          视频心跳上报
│   └── wbi.rs                WBI 签名算法
├── app/                      核心应用状态 & 事件循环
│   ├── mod.rs                App state (页面、侧边栏、凭证、主题等)
│   ├── actions.rs            动作分发 (48+ 动作)
│   ├── network_events.rs     网络事件处理
│   └── runtime.rs            主事件循环
├── application/              动作类型 & 网络工作线程
│   ├── action.rs             AppAction 枚举 (48 种)
│   └── network.rs            NetworkCommand/NetworkEvent + 后台线程
├── ui/                       TUI 组件 (13 个页面)
│   ├── mod.rs                Component trait、Page 枚举
│   ├── home.rs               推荐首页 (3列网格)
│   ├── search.rs             搜索 (8类型Tab + 热词)
│   ├── dynamic.rs            动态 (UP栏 + 3Tab)
│   ├── dynamic_detail.rs     动态详情 (图文 + 评论)
│   ├── video_detail.rs       视频详情 (信息 + 评论 + 选集 + 相关)
│   ├── video_card.rs         通用 VideoCard + VideoCardGrid
│   ├── history.rs            历史记录 (4列网格)
│   ├── live.rs               直播推荐 (3列网格)
│   ├── live_detail.rs        直播间 (信息 + WS弹幕)
│   ├── bangumi.rs            番剧索引 (番剧/国创)
│   ├── bangumi_detail.rs     番剧详情 (剧集分组)
│   ├── login.rs              二维码登录
│   ├── settings.rs           设置 (主题/键位/账号)
│   ├── sidebar.rs            左侧导航栏
│   ├── up_video_list.rs      UP主空间视频列表
│   └── theme.rs              主题系统 (39套)
├── player/                   MPV 播放器集成
│   └── mod.rs                play_video / play_bangumi / play_live
├── storage/                  持久化
│   └── mod.rs                Credentials、Keybindings、AppConfig
├── domain/                   类型别名
├── infrastructure/           外观层 (重导出)
└── presentation/             重导出层
```

## 数据流

```
用户输入 → UI 组件 → AppAction → App::handle_action()
                                          ↓
                                  NetworkCommand (异步请求)
                                          ↓
                                 后台网络工作线程 (独立 tokio)
                                          ↓
                                  NetworkEvent
                                          ↓
                                  App::handle_network_event()
                                          ↓
                                  页面状态更新 → 下一帧渲染
```

## 页面列表 (13个)

| 页面 | 组件 | 说明 |
|------|------|------|
| Login | `LoginPage` | 二维码登录 |
| Home | `HomePage` | 视频推荐 (3列) |
| Search | `SearchPage` | 搜索 (8类型) + 热词 |
| Dynamic | `DynamicPage` | 动态 (UP栏 + 3Tab) |
| DynamicDetail | `DynamicDetailPage` | 图文动态 + 评论 |
| VideoDetail | `VideoDetailPage` | 视频信息 + 评论 + 选集 + 相关 |
| History | `HistoryPage` | 历史记录 (4列) |
| Live | `LivePage` | 直播推荐 (3列) |
| LiveDetail | `LiveDetailPage` | 直播间 + 弹幕 |
| Bangumi | `BangumiPage` | 番剧索引 (番剧/国创) |
| BangumiDetail | `BangumiDetailPage` | 番剧详情 (剧集分组) |
| UpVideoList | `UpVideoListPage` | UP主空间视频列表 |
| Settings | `SettingsPage` | 主题/键位/账号 |

## 键位绑定 (24个)

| 绑定名 | 默认键 | 说明 |
|--------|--------|------|
| `quit` | `q` | 退出 |
| `confirm` | `Enter` | 确认/选择 |
| `back` | `Esc` | 返回 |
| `refresh` | `r` | 刷新 |
| `nav_up` | `k` / `↑` | 上移 |
| `nav_down` | `j` / `↓` | 下移 |
| `nav_left` | `h` / `←` | 左移 |
| `nav_right` | `l` / `→` | 右移 |
| `nav_next_page` | `Tab` | 下一侧栏页面 |
| `nav_prev_page` | `BackTab` | 上一侧栏页面 |
| `section_prev` | `[` | 上一分区 |
| `section_next` | `]` | 下一分区 |
| `tab_1`/`2`/`3` | `1`/`2`/`3` | 标签切换 |
| `next_theme` | `t` | 切换主题 |
| `play` | `p` | 播放 |
| `open_settings` | `s` | 设置 |
| `search_focus` | `/` | 搜索 |
| `comment` | `c` | 评论 |
| `toggle_replies` | `r` | 展开回复 |
| `up_prev` | `[` | 上一UP |
| `up_next` | `]` | 下一UP |
| `open_up` | `u` | 查看UP主 |

## 主题系统

- 基于 **Opaline** 的 39 套内置主题
- 涵盖 Catppuccin、Nord、Dracula、Tokyo Night、Rose Pine、Kanagawa 等流行配色
- 23 个语义色定义（背景、前景、边框、选中、状态色、Bilibili 品牌色）
- 默认主题：`silkcircuit-neon`

## 外部依赖

- **mpv** — 媒体播放器
- **yt-dlp** — 视频流提取

## 技术栈

| 依赖 | 用途 |
|------|------|
| ratatui 0.30 | TUI 框架 |
| crossterm 0.29 | 终端后端 |
| reqwest 0.13 | HTTP 客户端 |
| tokio 1.52 | 异步运行时 |
| tokio-tungstenite 0.29 | WebSocket |
| serde / serde_json | 序列化 |
| opaline 0.4 | 主题引擎 |
| ratatui-image 11 | 终端图片渲染 |
| qrcode / tui-qrcode | 二维码 |
| brotli / flate2 | 弹幕解压 |
