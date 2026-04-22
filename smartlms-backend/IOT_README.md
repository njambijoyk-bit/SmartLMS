# SmartLMS IoT Integration - Phase 14

## Overview

The SmartLMS IoT integration enables institutions to connect and manage various IoT devices for enhanced learning experiences, automated attendance tracking, environmental monitoring, and campus security.

## Features

### Supported Device Types

- **AttendanceScanner**: RFID/NFC readers for automated attendance tracking
- **EnvironmentalSensor**: Temperature, humidity, CO2, and air quality monitoring
- **SmartLock**: Access control for classrooms and facilities
- **MotionSensor**: Occupancy detection and security
- **EnergyMeter**: Power consumption monitoring
- **SecurityCamera**: Surveillance and AI proctoring integration
- **SmartDisplay**: Digital signage and classroom displays
- **EmergencyButton**: Panic buttons for emergency situations
- **LabEquipment**: Monitoring of laboratory equipment status
- **Generic**: Custom device support

### Core Capabilities

1. **Device Management**
   - Register and configure IoT devices
   - Monitor device status and health
   - Firmware version tracking
   - Device grouping for bulk operations

2. **Sensor Data Collection**
   - Real-time sensor reading submission
   - Time-series data storage optimized for analytics
   - Support for multiple sensor types per device
   - Metadata attachment for contextual information

3. **Command & Control**
   - Send commands to devices (lock/unlock, restart, configure)
   - Command queue with status tracking
   - Asynchronous command execution
   - Result capture and logging

4. **Alert System**
   - Configurable thresholds for sensor values
   - Multi-level severity (Low, Medium, High, Critical)
   - Real-time alert broadcasting via WebSocket
   - Alert resolution workflow
   - Notification integration

5. **Analytics**
   - Aggregated sensor statistics (min, max, avg)
   - Time-range queries for historical analysis
   - Device utilization metrics
   - Maintenance scheduling insights

## API Endpoints

### Device Management

```
POST   /api/iot/devices                      - Register new device
GET    /api/iot/devices/:institution_id      - List devices by institution
GET    /api/iot/devices/:id                  - Get device details
PUT    /api/iot/devices/:id/status           - Update device status
DELETE /api/iot/devices/:id                  - Remove device
```

### Sensor Readings

```
POST   /api/iot/devices/:id/readings         - Submit sensor reading
GET    /api/iot/devices/:id/readings/:limit  - Get recent readings
GET    /api/iot/analytics/:device_id/:sensor_type/:start/:end - Get analytics
```

### Commands

```
POST   /api/iot/devices/:id/commands         - Send command to device
GET    /api/iot/commands/:id                 - Get command status
```

### Alerts

```
POST   /api/iot/thresholds                   - Create alert threshold
GET    /api/iot/alerts/:device_id/:include_resolved - Get alerts
POST   /api/iot/alerts/:id/resolve           - Resolve an alert
```

### Utility

```
GET    /api/iot/health                       - Health check endpoint
```

## Database Schema

The IoT module uses 7 tables:

1. **iot_devices**: Device registry and metadata
2. **sensor_readings**: Time-series sensor data
3. **device_commands**: Command queue and execution tracking
4. **alert_thresholds**: Configurable alert rules
5. **iot_alerts**: Generated alerts
6. **iot_device_groups**: Logical device groupings
7. **iot_device_group_members**: Group membership
8. **device_maintenance_logs**: Maintenance history

### Migration

Run the migration to set up IoT tables:

```bash
psql -d smartlms -f migrations/007_iot_integration.sql
```

## Usage Examples

### Register a Device

```json
POST /api/iot/devices
{
  "name": "Classroom 101 Environmental Sensor",
  "device_type": "environmental_sensor",
  "location": "Building A, Room 101",
  "institution_id": "uuid-here",
  "course_id": "uuid-here",
  "ip_address": "192.168.1.100",
  "mac_address": "00:1A:2B:3C:4D:5E",
  "metadata": {
    "manufacturer": "SmartSense",
    "model": "ENV-2000"
  }
}
```

### Submit Sensor Reading

```json
POST /api/iot/devices/{device_id}/readings
{
  "sensor_type": "temperature",
  "value": 23.5,
  "unit": "celsius",
  "metadata": {
    "accuracy": 0.1
  }
}
```

### Send Command

```json
POST /api/iot/devices/{device_id}/commands
{
  "command_type": "lock",
  "parameters": {
    "timeout_seconds": 30
  }
}
```

### Create Alert Threshold

```json
POST /api/iot/thresholds
{
  "device_id": "uuid-here",
  "sensor_type": "temperature",
  "min_value": 18.0,
  "max_value": 26.0,
  "severity": "medium",
  "notification_enabled": true
}
```

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   IoT Devices   │────▶│  SmartLMS API    │────▶│   Database      │
│   (Sensors)     │◀────│  (IoT Service)   │◀────│  (Time-series)  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │
                               ▼
                        ┌──────────────────┐
                        │  Alert Broadcast │
                        │  (WebSocket)     │
                        └──────────────────┘
```

## Security Considerations

1. **Device Authentication**: Each device should have unique credentials
2. **IP Whitelisting**: Restrict device connections to known IP ranges
3. **Data Encryption**: All sensor data transmitted over TLS
4. **Access Control**: ABAC policies for device management
5. **Audit Logging**: All device operations logged for compliance

## Performance Optimization

- **Indexing**: Optimized indexes on device_id, timestamp, and sensor_type
- **Partitioning**: Consider TimescaleDB hypertables for large-scale deployments
- **Batch Inserts**: Support batch reading submission for high-frequency sensors
- **Data Retention**: Implement automatic old data archival

## Integration Points

- **Attendance Module**: Automatic attendance from RFID scanners
- **Facility Management**: Room occupancy and environmental controls
- **Security System**: Integration with access control and surveillance
- **Energy Management**: Power consumption tracking and optimization
- **AI Proctoring**: Camera integration for exam monitoring

## Future Enhancements

- MQTT protocol support for lightweight devices
- Edge computing capabilities for local processing
- Predictive maintenance using ML
- Digital twin visualization
- Mobile app for device management
- Voice assistant integration

## Troubleshooting

### Device Not Connecting

1. Verify network connectivity
2. Check device credentials
3. Ensure firewall allows IoT port
4. Review device logs

### Missing Sensor Readings

1. Check device battery/status
2. Verify sensor configuration
3. Review API rate limits
4. Check database connection

### Alerts Not Triggering

1. Verify threshold configuration
2. Check sensor data format
3. Review alert service logs
4. Ensure notification service is running

## Support

For issues or questions, contact the SmartLMS development team or refer to the main documentation.
