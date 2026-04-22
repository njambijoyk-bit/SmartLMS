# IoT Integration for SmartLMS

This document describes the Internet of Things (IoT) integration implemented for the SmartLMS platform.

## Overview

The IoT module enables SmartLMS to connect with physical devices in educational environments, including:
- **Attendance Scanners**: RFID/NFC readers for automated attendance tracking
- **Environmental Sensors**: Temperature, humidity, and air quality monitors
- **Smart Locks**: Access control for classrooms and labs
- **Motion Sensors**: Occupancy detection for space optimization
- **Energy Meters**: Power consumption monitoring
- **Security Cameras**: Surveillance and AI proctoring integration
- **Smart Displays**: Digital signage and information displays
- **Emergency Buttons**: Safety alert systems
- **Lab Equipment Monitors**: Tracking usage and status of laboratory equipment

## Architecture

### Components

1. **Models** (`src/models/iot.rs`)
   - Device types and status enums
   - IoT device entity
   - Sensor readings
   - Device commands
   - Alerts and thresholds

2. **Services** (`src/services/iot.rs`)
   - Device registration and management
   - Sensor data processing
   - Command queuing and execution
   - Alert threshold monitoring
   - Real-time alert broadcasting
   - Analytics aggregation

3. **API Endpoints** (`src/api/iot.rs`)
   - RESTful endpoints for all IoT operations
   - State management for service access
   - Request/response handling

4. **Database Schema** (`src/db/iot.rs`)
   - Table definitions for devices, readings, commands, and alerts
   - Optimized indexes for common queries
   - Migration functions

## API Endpoints

All IoT endpoints are prefixed with `/api/iot`

### Device Management

#### Register Device
```http
POST /api/iot/devices
Content-Type: application/json

{
  "name": "Classroom A Temp Sensor",
  "device_type": "environmental_sensor",
  "location": "Building 1, Room 101",
  "institution_id": "uuid-here",
  "course_id": "uuid-here",
  "room_id": "uuid-here",
  "ip_address": "192.168.1.100",
  "mac_address": "00:1B:44:11:3A:B7",
  "metadata": {"sensor_model": "DHT22"}
}
```

#### Get Device
```http
GET /api/iot/devices/{device_id}
```

#### List Devices
```http
GET /api/iot/devices/{institution_id}?device_type=environmental_sensor&status=online
```

#### Update Device Status
```http
PUT /api/iot/devices/{device_id}/status
Content-Type: application/json

{
  "status": "online",
  "firmware_version": "1.2.3"
}
```

#### Delete Device
```http
DELETE /api/iot/devices/{device_id}
```

### Sensor Readings

#### Submit Reading
```http
POST /api/iot/devices/{device_id}/readings
Content-Type: application/json

{
  "sensor_type": "temperature",
  "value": 23.5,
  "unit": "celsius",
  "metadata": {"battery_level": 85}
}
```

#### Get Recent Readings
```http
GET /api/iot/devices/{device_id}/readings/{limit}
```

### Device Commands

#### Send Command
```http
POST /api/iot/devices/{device_id}/commands
Content-Type: application/json

{
  "command_type": "lock_door",
  "parameters": {"duration_seconds": 30}
}
```

#### Get Command Status
```http
GET /api/iot/commands/{command_id}
```

### Alert Management

#### Create Alert Threshold
```http
POST /api/iot/thresholds
Content-Type: application/json

{
  "device_id": "uuid-here",
  "sensor_type": "temperature",
  "min_value": 18.0,
  "max_value": 26.0,
  "severity": "high",
  "notification_enabled": true
}
```

#### Get Alerts
```http
GET /api/iot/alerts/{device_id}/{include_resolved}
```

#### Resolve Alert
```http
POST /api/iot/alerts/{alert_id}/resolve
```

### Analytics

#### Get Sensor Analytics
```http
GET /api/iot/analytics/{device_id}/{sensor_type}/{start}/{end}
```

## Database Schema

### Tables

1. **iot_devices**: Stores registered IoT devices
   - Unique device identification
   - Type, status, and location
   - Network information (IP, MAC)
   - Metadata in JSONB format

2. **sensor_readings**: Time-series sensor data
   - Device reference
   - Timestamp, sensor type, value, unit
   - Optional metadata
   - Optimized for time-range queries

3. **device_commands**: Command queue for devices
   - Command type and parameters
   - Status tracking (pending, sent, executed, failed)
   - Execution results

4. **iot_alerts**: Alert notifications
   - Alert type and severity
   - Resolution status
   - Timestamps

5. **alert_thresholds**: Configurable thresholds
   - Min/max values per sensor type
   - Severity levels
   - Notification settings

## Use Cases

### 1. Automated Attendance
```rust
// Device: AttendanceScanner
// When a student scans their ID card:
POST /api/iot/devices/{scanner_id}/readings
{
  "sensor_type": "rfid_scan",
  "value": 0,
  "unit": "scan",
  "metadata": {
    "student_id": "STU-12345",
    "card_uid": "A1B2C3D4"
  }
}
// Backend triggers attendance recording
```

### 2. Environmental Monitoring
```rust
// Device: EnvironmentalSensor
// Periodic temperature/humidity reporting:
POST /api/iot/devices/{sensor_id}/readings
{
  "sensor_type": "temperature",
  "value": 24.5,
  "unit": "celsius"
}

// If threshold exceeded, automatic alert is generated
```

### 3. Smart Classroom Access
```rust
// Device: SmartLock
// Unlock door for scheduled class:
POST /api/iot/devices/{lock_id}/commands
{
  "command_type": "unlock",
  "parameters": {
    "duration_minutes": 90,
    "authorized_users": ["PROF-123"]
  }
}
```

### 4. Lab Equipment Monitoring
```rust
// Device: LabEquipment
// Track equipment usage:
POST /api/iot/devices/{equipment_id}/readings
{
  "sensor_type": "usage_hours",
  "value": 2.5,
  "unit": "hours",
  "metadata": {
    "experiment_id": "EXP-456",
    "user_id": "STU-789"
  }
}
```

### 5. Energy Management
```rust
// Device: EnergyMeter
// Monitor power consumption:
GET /api/iot/analytics/{meter_id}/power_consumption/2024-01-01T00:00:00Z/2024-01-31T23:59:59Z

// Returns aggregated min, max, avg consumption
```

## Integration Points

### With Attendance System
IoT attendance scanners automatically record student presence when they scan ID cards.

### With Course Management
Devices can be associated with specific courses for lab sessions or specialized classrooms.

### With Analytics Engine
Sensor data feeds into the analytics platform for insights on:
- Space utilization
- Energy efficiency
- Equipment usage patterns
- Environmental conditions

### With Notification System
Alerts trigger notifications to:
- Facility managers (maintenance issues)
- Instructors (room conditions)
- Security personnel (emergency buttons)

## Security Considerations

1. **Device Authentication**: Each device should have unique credentials
2. **Data Encryption**: All communications encrypted via TLS
3. **Access Control**: Role-based permissions for device management
4. **Audit Logging**: All device interactions logged for compliance
5. **Rate Limiting**: Prevent abuse of sensor data submission

## Future Enhancements

- **MQTT Broker Integration**: Real-time pub/sub messaging for devices
- **WebSocket Support**: Live sensor data streaming
- **Edge Computing**: Local processing for reduced latency
- **Machine Learning**: Anomaly detection in sensor patterns
- **Digital Twin**: Virtual representation of physical spaces
- **Blockchain**: Immutable audit trail for critical devices

## Testing

Example test scenarios:
1. Register a new device
2. Submit sensor readings
3. Verify alert generation on threshold breach
4. Send and track command execution
5. Query historical analytics

## Maintenance

- Regular firmware updates for devices
- Database cleanup for old sensor readings (retention policy)
- Monitoring device health and connectivity
- Reviewing and adjusting alert thresholds
