# Phase 15: WebSocket Real-time Features

## Overview

This phase implements comprehensive WebSocket support for real-time communication in SmartLMS, enabling instant notifications, live class interactions, collaborative features, and IoT device monitoring.

## Architecture

### Components

1. **WebSocketManager** (`src/services/websocket.rs`)
   - Connection management and tracking
   - Channel-based pub/sub system
   - User authentication and session handling
   - Broadcast capabilities

2. **API Endpoints** (`src/api/websocket.rs`)
   - WebSocket upgrade handlers
   - Statistics endpoint
   - Notification broadcast API

3. **Message Types**
   - Authentication
   - Channel subscription/unsubscription
   - Notifications
   - Live class events
   - Collaboration events
   - IoT updates
   - Error handling and acknowledgments

## Features

### 1. Real-time Notifications
- Instant delivery of system notifications
- Targeted user messaging
- Broadcast to all connected users
- Notification categorization

### 2. Live Class Support
- Real-time student presence tracking
- Instructor announcements
- Q&A sessions
- Polls and quizzes
- Screen sharing coordination

### 3. Collaboration Tools
- Shared whiteboard synchronization
- Collaborative code editing
- Document co-authoring
- Group project workspaces

### 4. IoT Integration
- Real-time sensor data streaming
- Device status updates
- Alert notifications
- Remote control commands

### 5. Channel System
- Public channels (course-wide)
- Private channels (1-on-1)
- Group channels (study groups)
- System channels (announcements)

## API Endpoints

### WebSocket Connections

#### Main WebSocket Endpoint
```
GET /api/ws
```
Establishes a WebSocket connection with auto-generated connection ID.

#### Channel-specific WebSocket
```
GET /api/ws/:channel
```
Connects to WebSocket and automatically subscribes to the specified channel.

### REST APIs

#### Get Connection Statistics
```
GET /api/ws/stats
```

Response:
```json
{
  "total_connections": 150,
  "authenticated_users": 120,
  "active_channels": 45,
  "channel_subscribers": {
    "live_class:123": 35,
    "notifications:user456": 1,
    "iot:sensor789": 5
  }
}
```

#### Broadcast Notification
```
POST /api/ws/notification
Content-Type: application/json

{
  "title": "System Maintenance",
  "message": "Scheduled maintenance tonight at 2 AM",
  "notification_type": "announcement",
  "data": {
    "priority": "high",
    "scheduled_time": "2024-01-15T02:00:00Z"
  }
}
```

Response:
```json
{
  "success": true,
  "recipients": 120
}
```

## Message Protocol

### Client → Server Messages

#### Authentication
```json
{
  "type": "Auth",
  "token": "jwt_token_here"
}
```

#### Subscribe to Channel
```json
{
  "type": "Subscribe",
  "channel": "live_class:abc123"
}
```

#### Unsubscribe from Channel
```json
{
  "type": "Unsubscribe",
  "channel": "live_class:abc123"
}
```

#### Send Message to Channel
```json
{
  "type": "Send",
  "channel": "collab:room456",
  "payload": {
    "event": "whiteboard_draw",
    "data": {"x": 100, "y": 200, "color": "#FF0000"}
  }
}
```

### Server → Client Messages

#### Receive Message
```json
{
  "type": "Receive",
  "channel": "collab:room456",
  "payload": {
    "event": "whiteboard_draw",
    "user_id": "user123",
    "data": {"x": 100, "y": 200, "color": "#FF0000"}
  },
  "timestamp": 1705312800
}
```

#### Notification
```json
{
  "type": "Notification",
  "title": "New Assignment",
  "message": "Assignment 'Essay' is due tomorrow",
  "notification_type": "assignment",
  "data": {
    "assignment_id": "789",
    "course_id": "456"
  }
}
```

#### Live Class Event
```json
{
  "type": "LiveClass",
  "class_id": "class123",
  "event": "student_joined",
  "data": {
    "student_id": "user456",
    "student_name": "John Doe"
  }
}
```

#### Collaboration Event
```json
{
  "type": "Collaboration",
  "room_id": "room789",
  "event": "cursor_move",
  "user_id": "user123",
  "data": {
    "x": 150,
    "y": 300,
    "cursor_color": "#00FF00"
  }
}
```

#### IoT Update
```json
{
  "type": "IotUpdate",
  "device_id": "sensor001",
  "sensor_type": "temperature",
  "value": 23.5,
  "timestamp": 1705312800
}
```

#### Error
```json
{
  "type": "Error",
  "code": "AUTH_FAILED",
  "message": "Invalid authentication token"
}
```

#### Acknowledgment
```json
{
  "type": "Ack",
  "message_id": "msg123",
  "status": "delivered"
}
```

## Usage Examples

### JavaScript Client

```javascript
// Connect to WebSocket
const ws = new WebSocket('ws://localhost:8000/api/ws');

ws.onopen = () => {
  console.log('Connected to WebSocket');
  
  // Authenticate
  ws.send(JSON.stringify({
    type: 'Auth',
    token: localStorage.getItem('jwt_token')
  }));
  
  // Subscribe to course channel
  ws.send(JSON.stringify({
    type: 'Subscribe',
    channel: 'live_class:MATH101'
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  
  switch(message.type) {
    case 'Notification':
      showNotification(message.title, message.message);
      break;
    case 'LiveClass':
      handleLiveClassEvent(message);
      break;
    case 'IotUpdate':
      updateSensorDisplay(message);
      break;
  }
};

// Send collaboration event
function sendCollaborationEvent(roomId, eventData) {
  ws.send(JSON.stringify({
    type: 'Send',
    channel: `collab:${roomId}`,
    payload: eventData
  }));
}
```

### React Hook Example

```javascript
import { useEffect, useState, useCallback } from 'react';

export function useWebSocket(channel) {
  const [messages, setMessages] = useState([]);
  const [connected, setConnected] = useState(false);
  
  useEffect(() => {
    const ws = new WebSocket(`ws://localhost:8000/api/ws/${channel}`);
    
    ws.onopen = () => {
      setConnected(true);
      ws.send(JSON.stringify({
        type: 'Auth',
        token: localStorage.getItem('jwt_token')
      }));
    };
    
    ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      setMessages(prev => [...prev, message]);
    };
    
    ws.onclose = () => setConnected(false);
    
    return () => ws.close();
  }, [channel]);
  
  const sendMessage = useCallback((payload) => {
    ws.send(JSON.stringify({
      type: 'Send',
      channel,
      payload
    }));
  }, [channel]);
  
  return { messages, connected, sendMessage };
}
```

## Performance Considerations

### Scalability
- Horizontal scaling with Redis pub/sub backend (future enhancement)
- Connection pooling and reuse
- Message batching for high-frequency updates
- Compression for large payloads

### Security
- JWT token authentication required
- Rate limiting per connection
- Channel access control based on user permissions
- Input validation and sanitization
- TLS/SSL encryption in production

### Monitoring
- Connection count tracking
- Message throughput metrics
- Latency monitoring
- Error rate tracking
- Channel activity analytics

## Testing

Run unit tests:
```bash
cargo test websocket
```

Integration test example:
```bash
# Start server
cargo run

# In another terminal, use wscat or similar tool
wscat -c ws://localhost:8000/api/ws
```

## Future Enhancements

1. **Redis Integration**: For multi-server deployments
2. **Presence API**: Track online/offline status
3. **Message History**: Persistent chat history
4. **File Transfer**: Binary data support
5. **Video/Audio**: WebRTC signaling integration
6. **GraphQL Subscriptions**: Alternative to WebSocket
7. **Server-Sent Events**: For simpler use cases

## Migration Guide

If migrating from polling-based updates:

1. Replace polling intervals with WebSocket subscriptions
2. Implement reconnection logic with exponential backoff
3. Add offline message queue
4. Handle connection state changes gracefully
5. Implement message acknowledgment for critical updates

## Troubleshooting

### Common Issues

**Connection drops frequently:**
- Check network stability
- Implement ping/pong heartbeat
- Increase timeout values
- Add automatic reconnection

**Messages not received:**
- Verify channel subscription
- Check authentication status
- Inspect server logs for errors
- Ensure proper message format

**High latency:**
- Monitor server load
- Check network conditions
- Implement message prioritization
- Consider edge computing for IoT

## Conclusion

Phase 15 provides a robust WebSocket infrastructure enabling real-time features across SmartLMS. The implementation supports multiple use cases from simple notifications to complex collaborative applications, with built-in scalability and security features.

Next phases will build upon this foundation to add mobile app enhancements (Phase 16), advanced analytics (Phase 17), and blockchain certificates (Phase 18).
