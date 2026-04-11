// Phase 7: Live Virtual Classroom API - WebRTC-based video conferencing
// Provides endpoints for live classes with interactive features

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::models::live::*;

// ==================== WebRTC Signaling Structures ====================

/// WebRTC offer for establishing peer connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcOffer {
    pub sdp: String,
    pub session_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Option<Uuid>, // None for broadcast
}

/// WebRTC answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcAnswer {
    pub sdp: String,
    pub session_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
}

/// ICE candidate for NAT traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCandidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_mline_index: i32,
    pub session_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Option<Uuid>,
}

// ==================== Interactive Whiteboard ====================

/// Whiteboard session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardSession {
    pub id: Uuid,
    pub live_session_id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub background_color: String,
    pub elements: Vec<WhiteboardElement>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Whiteboard element (drawing, text, shape, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WhiteboardElement {
    #[serde(rename = "line")]
    Line {
        id: String,
        points: Vec<(f64, f64)>,
        color: String,
        width: f64,
        created_by: Uuid,
    },
    #[serde(rename = "rectangle")]
    Rectangle {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: String,
        filled: bool,
        created_by: Uuid,
    },
    #[serde(rename = "circle")]
    Circle {
        id: String,
        center_x: f64,
        center_y: f64,
        radius: f64,
        color: String,
        filled: bool,
        created_by: Uuid,
    },
    #[serde(rename = "text")]
    Text {
        id: String,
        x: f64,
        y: f64,
        content: String,
        font_size: i32,
        color: String,
        created_by: Uuid,
    },
    #[serde(rename = "image")]
    Image {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        url: String,
        created_by: Uuid,
    },
    #[serde(rename = "sticky_note")]
    StickyNote {
        id: String,
        x: f64,
        y: f64,
        content: String,
        color: String,
        created_by: Uuid,
    },
}

/// Whiteboard operation (add, update, delete)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum WhiteboardOperation {
    #[serde(rename = "add")]
    Add { element: WhiteboardElement },
    #[serde(rename = "update")]
    Update { id: String, element: WhiteboardElement },
    #[serde(rename = "delete")]
    Delete { id: String },
    #[serde(rename = "clear")]
    Clear,
}

/// Request to create whiteboard
#[derive(Debug, Deserialize)]
pub struct CreateWhiteboardRequest {
    pub live_session_id: Uuid,
    pub title: String,
    pub background_color: Option<String>,
}

// ==================== Screen Sharing ====================

/// Screen share stream info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenShareStream {
    pub id: Uuid,
    pub session_id: Uuid,
    pub sharer_user_id: Uuid,
    pub stream_type: ScreenShareType,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenShareType {
    EntireScreen,
    Window,
    BrowserTab,
}

/// Request to start screen sharing
#[derive(Debug, Deserialize)]
pub struct StartScreenShareRequest {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub stream_type: ScreenShareType,
}

// ==================== Breakout Rooms ====================

/// Breakout room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakoutRoom {
    pub id: Uuid,
    pub parent_session_id: Uuid,
    pub name: String,
    pub instructor_id: Uuid,
    pub participants: Vec<Uuid>,
    pub max_participants: i32,
    pub status: BreakoutRoomStatus,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakoutRoomStatus {
    Created,
    Active,
    Ended,
}

/// Request to create breakout rooms
#[derive(Debug, Deserialize)]
pub struct CreateBreakoutRoomsRequest {
    pub session_id: Uuid,
    pub room_count: i32,
    pub assignment_mode: RoomAssignmentMode,
    pub participants: Vec<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomAssignmentMode {
    Automatic,
    Manual,
    SelfSelect,
}

/// Assign participants to rooms
#[derive(Debug, Deserialize)]
pub struct AssignToRoomRequest {
    pub room_id: Uuid,
    pub participant_ids: Vec<Uuid>,
}

// ==================== Real-time Polls & Quizzes ====================

/// Live poll
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivePoll {
    pub id: Uuid,
    pub session_id: Uuid,
    pub created_by: Uuid,
    pub question: String,
    pub poll_type: PollType,
    pub options: Vec<PollOption>,
    pub responses: HashMap<Uuid, Uuid>, // user_id -> option_id
    pub is_active: bool,
    pub show_results_live: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PollType {
    SingleChoice,
    MultipleChoice,
    TrueFalse,
    Rating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub id: Uuid,
    pub text: String,
    pub color: Option<String>,
}

/// Request to create poll
#[derive(Debug, Deserialize)]
pub struct CreatePollRequest {
    pub session_id: Uuid,
    pub question: String,
    pub poll_type: PollType,
    pub options: Vec<String>,
    pub show_results_live: bool,
}

/// Submit poll response
#[derive(Debug, Deserialize)]
pub struct SubmitPollResponse {
    pub poll_id: Uuid,
    pub option_ids: Vec<Uuid>,
}

// ==================== Session Recording ====================

/// Recording metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecording {
    pub id: Uuid,
    pub session_id: Uuid,
    pub recording_url: String,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: i64,
    pub file_size_mb: f64,
    pub recording_type: RecordingType,
    pub status: RecordingStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingType {
    VideoOnly,
    AudioOnly,
    ScreenShare,
    Whiteboard,
    FullSession,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingStatus {
    Recording,
    Processing,
    Ready,
    Failed,
}

// ==================== In-Memory State for Real-time Features ====================

/// Shared state for WebSocket connections and real-time data
#[derive(Debug, Clone)]
pub struct LiveClassState {
    /// Active WebSocket connections: session_id -> user_id -> connection_id
    pub active_connections: std::sync::Arc<RwLock<HashMap<Uuid, HashMap<Uuid, String>>>>,
    /// Pending WebRTC offers
    pub pending_offers: std::sync::Arc<RwLock<Vec<WebRtcOffer>>>,
    /// Active screen shares
    pub active_screen_shares: std::sync::Arc<RwLock<HashMap<Uuid, ScreenShareStream>>>,
    /// Whiteboard states
    pub whiteboards: std::sync::Arc<RwLock<HashMap<Uuid, WhiteboardSession>>>,
    /// Active polls
    pub active_polls: std::sync::Arc<RwLock<HashMap<Uuid, LivePoll>>>,
    /// Breakout rooms
    pub breakout_rooms: std::sync::Arc<RwLock<HashMap<Uuid, BreakoutRoom>>>,
}

impl Default for LiveClassState {
    fn default() -> Self {
        Self {
            active_connections: std::sync::Arc::new(RwLock::new(HashMap::new())),
            pending_offers: std::sync::Arc::new(RwLock::new(Vec::new())),
            active_screen_shares: std::sync::Arc::new(RwLock::new(HashMap::new())),
            whiteboards: std::sync::Arc::new(RwLock::new(HashMap::new())),
            active_polls: std::sync::Arc::new(RwLock::new(HashMap::new())),
            breakout_rooms: std::sync::Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// ==================== API Routes ====================

pub fn live_router() -> Router {
    Router::new()
        .route("/sessions/:session_id/join", axum::routing::post(handle_join_session))
        .route("/sessions/:session_id/leave", axum::routing::post(handle_leave_session))
        .route("/sessions/:session_id/participants", axum::routing::get(handle_get_participants))
        // WebRTC signaling
        .route("/webrtc/offer", axum::routing::post(handle_send_offer))
        .route("/webrtc/answer", axum::routing::post(handle_send_answer))
        .route("/webrtc/ice", axum::routing::post(handle_send_ice_candidate))
        // Whiteboard
        .route("/whiteboard", axum::routing::post(handle_create_whiteboard))
        .route("/whiteboard/:whiteboard_id", axum::routing::get(handle_get_whiteboard))
        .route("/whiteboard/:whiteboard_id/operate", axum::routing::post(handle_whiteboard_operation))
        // Screen sharing
        .route("/screen-share/start", axum::routing::post(handle_start_screen_share))
        .route("/screen-share/stop", axum::routing::post(handle_stop_screen_share))
        .route("/screen-share/active", axum::routing::get(handle_get_active_screen_shares))
        // Breakout rooms
        .route("/breakout-rooms/create", axum::routing::post(handle_create_breakout_rooms))
        .route("/breakout-rooms/:room_id/assign", axum::routing::post(handle_assign_to_room))
        .route("/breakout-rooms/:room_id/join", axum::routing::post(handle_join_breakout_room))
        .route("/breakout-rooms/:room_id/leave", axum::routing::post(handle_leave_breakout_room))
        .route("/breakout-rooms/end-all", axum::routing::post(handle_end_all_breakout_rooms))
        // Polls & quizzes
        .route("/polls/create", axum::routing::post(handle_create_poll))
        .route("/polls/:poll_id/submit", axum::routing::post(handle_submit_poll))
        .route("/polls/:poll_id/results", axum::routing::get(handle_get_poll_results))
        .route("/polls/:poll_id/close", axum::routing::post(handle_close_poll))
        // Recordings
        .route("/recordings/start", axum::routing::post(handle_start_recording))
        .route("/recordings/stop", axum::routing::post(handle_stop_recording))
        .route("/recordings/:session_id", axum::routing::get(handle_get_recordings))
}

// ==================== Handler Implementations ====================

async fn handle_join_session(
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<JoinSessionRequest>,
) -> Result<Json<JoinSessionResponse>, StatusCode> {
    // Verify user is enrolled in the course
    // Update session participant count
    // Return join URL and credentials
    
    Ok(Json(JoinSessionResponse {
        success: true,
        join_url: format!("https://meet.smartlms.com/{}", session_id),
        access_token: "mock_token".to_string(),
        error: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct JoinSessionRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct JoinSessionResponse {
    pub success: bool,
    pub join_url: String,
    pub access_token: String,
    pub error: Option<String>,
}

async fn handle_leave_session(
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<LeaveSessionRequest>,
) -> Result<Json<LeaveSessionResponse>, StatusCode> {
    // Record leave time
    // Update attendance
    
    Ok(Json(LeaveSessionResponse {
        success: true,
    }))
}

#[derive(Debug, Deserialize)]
pub struct LeaveSessionRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct LeaveSessionResponse {
    pub success: bool,
}

async fn handle_get_participants(
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<Vec<ParticipantInfo>>, StatusCode> {
    // Get current participants in the session
    
    Ok(Json(Vec::new()))
}

#[derive(Debug, Serialize)]
pub struct ParticipantInfo {
    pub user_id: Uuid,
    pub name: String,
    pub role: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub is_speaking: bool,
    pub has_video: bool,
    pub has_hand_raised: bool,
}

// WebRTC Signaling Handlers
async fn handle_send_offer(
    State(state): State<LiveClassState>,
    Json(offer): Json<WebRtcOffer>,
) -> Result<Json<SignalingResponse>, StatusCode> {
    // Store offer for recipient to pick up
    let mut pending = state.pending_offers.write().await;
    pending.push(offer.clone());
    
    // In production, this would be sent via WebSocket to the specific recipient
    
    Ok(Json(SignalingResponse {
        success: true,
        message: "Offer sent".to_string(),
    }))
}

async fn handle_send_answer(
    State(state): State<LiveClassState>,
    Json(answer): Json<WebRtcAnswer>,
) -> Result<Json<SignalingResponse>, StatusCode> {
    // Forward answer to original offer creator
    
    Ok(Json(SignalingResponse {
        success: true,
        message: "Answer sent".to_string(),
    }))
}

async fn handle_send_ice_candidate(
    State(state): State<LiveClassState>,
    Json(candidate): Json<IceCandidate>,
) -> Result<Json<SignalingResponse>, StatusCode> {
    // Forward ICE candidate
    
    Ok(Json(SignalingResponse {
        success: true,
        message: "ICE candidate sent".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct SignalingResponse {
    pub success: bool,
    pub message: String,
}

// Whiteboard Handlers
async fn handle_create_whiteboard(
    State(state): State<LiveClassState>,
    Json(req): Json<CreateWhiteboardRequest>,
) -> Result<Json<WhiteboardSession>, StatusCode> {
    let whiteboard = WhiteboardSession {
        id: Uuid::new_v4(),
        live_session_id: req.live_session_id,
        created_by: Uuid::new_v4(), // Should come from auth context
        title: req.title,
        background_color: req.background_color.unwrap_or("#FFFFFF".to_string()),
        elements: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let mut whiteboards = state.whiteboards.write().await;
    whiteboards.insert(whiteboard.id, whiteboard.clone());
    
    Ok(Json(whiteboard))
}

async fn handle_get_whiteboard(
    State(state): State<LiveClassState>,
    Path(whiteboard_id): Path<Uuid>,
) -> Result<Json<WhiteboardSession>, StatusCode> {
    let whiteboards = state.whiteboards.read().await;
    
    whiteboards
        .get(&whiteboard_id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn handle_whiteboard_operation(
    State(state): State<LiveClassState>,
    Path(whiteboard_id): Path<Uuid>,
    Json(operation): Json<WhiteboardOperation>,
) -> Result<Json<WhiteboardSession>, StatusCode> {
    let mut whiteboards = state.whiteboards.write().await;
    
    let whiteboard = whiteboards
        .get_mut(&whiteboard_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    match operation {
        WhiteboardOperation::Add { element } => {
            whiteboard.elements.push(element);
        }
        WhiteboardOperation::Update { id, element } => {
            if let Some(pos) = whiteboard.elements.iter().position(|e| {
                match e {
                    WhiteboardElement::Line { id: eid, .. } => eid == &id,
                    WhiteboardElement::Rectangle { id: eid, .. } => eid == &id,
                    WhiteboardElement::Circle { id: eid, .. } => eid == &id,
                    WhiteboardElement::Text { id: eid, .. } => eid == &id,
                    WhiteboardElement::Image { id: eid, .. } => eid == &id,
                    WhiteboardElement::StickyNote { id: eid, .. } => eid == &id,
                }
            }) {
                whiteboard.elements[pos] = element;
            }
        }
        WhiteboardOperation::Delete { id } => {
            whiteboard.elements.retain(|e| {
                match e {
                    WhiteboardElement::Line { id: eid, .. } => eid != &id,
                    WhiteboardElement::Rectangle { id: eid, .. } => eid != &id,
                    WhiteboardElement::Circle { id: eid, .. } => eid != &id,
                    WhiteboardElement::Text { id: eid, .. } => eid != &id,
                    WhiteboardElement::Image { id: eid, .. } => eid != &id,
                    WhiteboardElement::StickyNote { id: eid, .. } => eid != &id,
                }
            });
        }
        WhiteboardOperation::Clear => {
            whiteboard.elements.clear();
        }
    }
    
    whiteboard.updated_at = chrono::Utc::now();
    
    Ok(Json(whiteboard.clone()))
}

// Screen Share Handlers
async fn handle_start_screen_share(
    State(state): State<LiveClassState>,
    Json(req): Json<StartScreenShareRequest>,
) -> Result<Json<ScreenShareStream>, StatusCode> {
    // Stop any existing screen share in this session
    {
        let mut shares = state.active_screen_shares.write().await;
        if let Some(existing) = shares.get_mut(&req.session_id) {
            if existing.is_active {
                existing.ended_at = Some(chrono::Utc::now());
                existing.is_active = false;
            }
        }
    }
    
    let stream = ScreenShareStream {
        id: Uuid::new_v4(),
        session_id: req.session_id,
        sharer_user_id: req.user_id,
        stream_type: req.stream_type,
        started_at: chrono::Utc::now(),
        ended_at: None,
        is_active: true,
    };
    
    let mut shares = state.active_screen_shares.write().await;
    shares.insert(req.session_id, stream.clone());
    
    Ok(Json(stream))
}

async fn handle_stop_screen_share(
    State(state): State<LiveClassState>,
    Json(req): Json<StopScreenShareRequest>,
) -> Result<Json<StopScreenShareResponse>, StatusCode> {
    let mut shares = state.active_screen_shares.write().await;
    
    if let Some(stream) = shares.get_mut(&req.session_id) {
        stream.ended_at = Some(chrono::Utc::now());
        stream.is_active = false;
        
        return Ok(Json(StopScreenShareResponse {
            success: true,
            message: "Screen sharing stopped".to_string(),
        }));
    }
    
    Err(StatusCode::NOT_FOUND)
}

#[derive(Debug, Deserialize)]
pub struct StopScreenShareRequest {
    pub session_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct StopScreenShareResponse {
    pub success: bool,
    pub message: String,
}

async fn handle_get_active_screen_shares(
    State(state): State<LiveClassState>,
    Query(req): Query<ActiveScreenSharesRequest>,
) -> Result<Json<Vec<ScreenShareStream>>, StatusCode> {
    let shares = state.active_screen_shares.read().await;
    
    let active: Vec<ScreenShareStream> = shares
        .values()
        .filter(|s| s.is_active && s.session_id == req.session_id)
        .cloned()
        .collect();
    
    Ok(Json(active))
}

#[derive(Debug, Deserialize)]
pub struct ActiveScreenSharesRequest {
    pub session_id: Uuid,
}

// Breakout Room Handlers
async fn handle_create_breakout_rooms(
    State(state): State<LiveClassState>,
    Json(req): Json<CreateBreakoutRoomsRequest>,
) -> Result<Json<Vec<BreakoutRoom>>, StatusCode> {
    let mut rooms = Vec::new();
    
    for i in 0..req.room_count {
        let room = BreakoutRoom {
            id: Uuid::new_v4(),
            parent_session_id: req.session_id,
            name: format!("Room {}", i + 1),
            instructor_id: Uuid::new_v4(), // Should come from context
            participants: Vec::new(),
            max_participants: ((req.participants.len() as i32) / req.room_count) + 1,
            status: BreakoutRoomStatus::Created,
            started_at: None,
            ended_at: None,
        };
        rooms.push(room);
    }
    
    // Auto-assign participants if automatic mode
    if req.assignment_mode == RoomAssignmentMode::Automatic {
        for (i, participant_id) in req.participants.iter().enumerate() {
            let room_idx = i % rooms.len();
            rooms[room_idx].participants.push(*participant_id);
        }
    }
    
    let mut breakout_rooms = state.breakout_rooms.write().await;
    for room in &rooms {
        breakout_rooms.insert(room.id, room.clone());
    }
    
    Ok(Json(rooms))
}

async fn handle_assign_to_room(
    State(state): State<LiveClassState>,
    Path(room_id): Path<Uuid>,
    Json(req): Json<AssignToRoomRequest>,
) -> Result<Json<AssignToRoomResponse>, StatusCode> {
    let mut breakout_rooms = state.breakout_rooms.write().await;
    
    let room = breakout_rooms
        .get_mut(&room_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    room.participants.extend(req.participant_ids);
    
    Ok(Json(AssignToRoomResponse {
        success: true,
        participant_count: room.participants.len(),
    }))
}

#[derive(Debug, Serialize)]
pub struct AssignToRoomResponse {
    pub success: bool,
    pub participant_count: usize,
}

async fn handle_join_breakout_room(
    State(state): State<LiveClassState>,
    Path(room_id): Path<Uuid>,
    Json(req): Json<JoinBreakoutRoomRequest>,
) -> Result<Json<JoinBreakoutRoomResponse>, StatusCode> {
    let mut breakout_rooms = state.breakout_rooms.write().await;
    
    let room = breakout_rooms
        .get_mut(&room_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if !room.participants.contains(&req.user_id) {
        room.participants.push(req.user_id);
    }
    
    room.status = BreakoutRoomStatus::Active;
    if room.started_at.is_none() {
        room.started_at = Some(chrono::Utc::now());
    }
    
    Ok(Json(JoinBreakoutRoomResponse {
        success: true,
        room_name: room.name.clone(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct JoinBreakoutRoomRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct JoinBreakoutRoomResponse {
    pub success: bool,
    pub room_name: String,
}

async fn handle_leave_breakout_room(
    State(state): State<LiveClassState>,
    Path(room_id): Path<Uuid>,
    Json(req): Json<LeaveBreakoutRoomRequest>,
) -> Result<Json<LeaveBreakoutRoomResponse>, StatusCode> {
    let mut breakout_rooms = state.breakout_rooms.write().await;
    
    let room = breakout_rooms
        .get_mut(&room_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    room.participants.retain(|id| id != &req.user_id);
    
    // If no participants left, mark as ended
    if room.participants.is_empty() {
        room.status = BreakoutRoomStatus::Ended;
        room.ended_at = Some(chrono::Utc::now());
    }
    
    Ok(Json(LeaveBreakoutRoomResponse {
        success: true,
    }))
}

#[derive(Debug, Deserialize)]
pub struct LeaveBreakoutRoomRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct LeaveBreakoutRoomResponse {
    pub success: bool,
}

async fn handle_end_all_breakout_rooms(
    State(state): State<LiveClassState>,
    Json(req): Json<EndAllBreakoutRoomsRequest>,
) -> Result<Json<EndAllBreakoutRoomsResponse>, StatusCode> {
    let mut breakout_rooms = state.breakout_rooms.write().await;
    
    let now = chrono::Utc::now();
    
    for (_, room) in breakout_rooms.iter_mut() {
        if room.parent_session_id == req.session_id && room.status == BreakoutRoomStatus::Active {
            room.status = BreakoutRoomStatus::Ended;
            room.ended_at = Some(now);
        }
    }
    
    Ok(Json(EndAllBreakoutRoomsResponse {
        success: true,
        message: "All breakout rooms ended".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct EndAllBreakoutRoomsRequest {
    pub session_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct EndAllBreakoutRoomsResponse {
    pub success: bool,
    pub message: String,
}

// Poll Handlers
async fn handle_create_poll(
    State(state): State<LiveClassState>,
    Json(req): Json<CreatePollRequest>,
) -> Result<Json<LivePoll>, StatusCode> {
    let options: Vec<PollOption> = req.options
        .into_iter()
        .enumerate()
        .map(|(i, text)| PollOption {
            id: Uuid::new_v4(),
            text,
            color: None,
        })
        .collect();
    
    let poll = LivePoll {
        id: Uuid::new_v4(),
        session_id: req.session_id,
        created_by: Uuid::new_v4(), // Should come from context
        question: req.question,
        poll_type: req.poll_type,
        options,
        responses: HashMap::new(),
        is_active: true,
        show_results_live: req.show_results_live,
        created_at: chrono::Utc::now(),
    };
    
    let mut polls = state.active_polls.write().await;
    polls.insert(poll.id, poll.clone());
    
    Ok(Json(poll))
}

async fn handle_submit_poll(
    State(state): State<LiveClassState>,
    Path(poll_id): Path<Uuid>,
    Json(req): Json<SubmitPollResponse>,
) -> Result<Json<SubmitPollResult>, StatusCode> {
    let mut polls = state.active_polls.write().await;
    
    let poll = polls
        .get_mut(&poll_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if !poll.is_active {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Record response (for single choice, use first option)
    if let Some(option_id) = req.option_ids.first() {
        poll.responses.insert(Uuid::new_v4(), *option_id); // User ID should come from context
    }
    
    Ok(Json(SubmitPollResult {
        success: true,
        total_responses: poll.responses.len(),
    }))
}

#[derive(Debug, Serialize)]
pub struct SubmitPollResult {
    pub success: bool,
    pub total_responses: usize,
}

async fn handle_get_poll_results(
    State(state): State<LiveClassState>,
    Path(poll_id): Path<Uuid>,
) -> Result<Json<PollResults>, StatusCode> {
    let polls = state.active_polls.read().await;
    
    let poll = polls
        .get(&poll_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Count votes per option
    let mut option_counts: HashMap<Uuid, i32> = HashMap::new();
    for option in &poll.options {
        option_counts.insert(option.id, 0);
    }
    
    for (_, option_id) in &poll.responses {
        if let Some(count) = option_counts.get_mut(option_id) {
            *count += 1;
        }
    }
    
    let total_votes = poll.responses.len();
    
    Ok(Json(PollResults {
        poll_id: poll.id,
        question: poll.question.clone(),
        options: poll.options.iter().map(|o| PollResultOption {
            id: o.id,
            text: o.text.clone(),
            votes: *option_counts.get(&o.id).unwrap_or(&0),
            percentage: if total_votes > 0 {
                (*option_counts.get(&o.id).unwrap_or(&0) as f64 / total_votes as f64) * 100.0
            } else {
                0.0
            },
        }).collect(),
        total_votes,
        is_active: poll.is_active,
    }))
}

#[derive(Debug, Serialize)]
pub struct PollResults {
    pub poll_id: Uuid,
    pub question: String,
    pub options: Vec<PollResultOption>,
    pub total_votes: usize,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct PollResultOption {
    pub id: Uuid,
    pub text: String,
    pub votes: i32,
    pub percentage: f64,
}

async fn handle_close_poll(
    State(state): State<LiveClassState>,
    Path(poll_id): Path<Uuid>,
) -> Result<Json<ClosePollResponse>, StatusCode> {
    let mut polls = state.active_polls.write().await;
    
    let poll = polls
        .get_mut(&poll_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    poll.is_active = false;
    
    Ok(Json(ClosePollResponse {
        success: true,
        final_vote_count: poll.responses.len(),
    }))
}

#[derive(Debug, Serialize)]
pub struct ClosePollResponse {
    pub success: bool,
    pub final_vote_count: usize,
}

// Recording Handlers
async fn handle_start_recording(
    State(pool): State<PgPool>,
    Json(req): Json<StartRecordingRequest>,
) -> Result<Json<SessionRecording>, StatusCode> {
    // Create recording record
    // Start actual recording process (would integrate with media server)
    
    let recording = SessionRecording {
        id: Uuid::new_v4(),
        session_id: req.session_id,
        recording_url: String::new(),
        thumbnail_url: None,
        duration_seconds: 0,
        file_size_mb: 0.0,
        recording_type: req.recording_type,
        status: RecordingStatus::Recording,
        created_at: chrono::Utc::now(),
        processed_at: None,
    };
    
    // TODO: Save to database
    
    Ok(Json(recording))
}

#[derive(Debug, Deserialize)]
pub struct StartRecordingRequest {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub recording_type: RecordingType,
}

async fn handle_stop_recording(
    State(pool): State<PgPool>,
    Json(req): Json<StopRecordingRequest>,
) -> Result<Json<StopRecordingResponse>, StatusCode> {
    // Stop recording and trigger processing
    
    Ok(Json(StopRecordingResponse {
        success: true,
        message: "Recording stopped and queued for processing".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct StopRecordingRequest {
    pub session_id: Uuid,
    pub recording_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct StopRecordingResponse {
    pub success: bool,
    pub message: String,
}

async fn handle_get_recordings(
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<Vec<SessionRecording>>, StatusCode> {
    // Fetch all recordings for a session
    
    Ok(Json(Vec::new()))
}
