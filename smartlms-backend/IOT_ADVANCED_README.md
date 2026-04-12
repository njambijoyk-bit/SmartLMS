# IoT Advanced Features Implementation

## Overview

This implementation adds advanced IoT capabilities to SmartLMS including:
- **MQTT Integration**: Real-time pub/sub messaging for IoT devices
- **Edge Computing**: Local data processing and rule execution
- **Predictive Maintenance**: ML-based failure prediction and maintenance scheduling

## Files Created

### Services Layer
1. `src/services/mqtt.rs` - MQTT client management
2. `src/services/edge_computing.rs` - Edge node processing
3. `src/services/predictive_maintenance.rs` - ML-based maintenance prediction

### API Layer
4. `src/api/iot_advanced.rs` - REST API endpoints

### Configuration
5. `Cargo.toml` - Updated with new dependencies

## Dependencies Added

```toml
# MQTT Client
rumqttc = "0.24"

# Edge Computing & Message Queue
tokio-stream = "0.1"
flume = "0.11"

# Machine Learning for Predictive Maintenance
linfa = "0.7"
linfa-trees = "0.7"
ndarray = "0.15"
```

## API Endpoints

### MQTT Management
- `POST /api/iot-advanced/mqtt/config` - Configure MQTT connection
- `GET /api/iot-advanced/mqtt/status` - Get MQTT connection status

### Edge Computing
- `POST /api/iot-advanced/edge/nodes` - Register edge node
- `GET /api/iot-advanced/edge/nodes` - List edge nodes
- `POST /api/iot-advanced/edge/rules` - Add processing rule

### Predictive Maintenance
- `POST /api/iot-advanced/maintenance/analyze` - Analyze device health
- `GET /api/iot-advanced/maintenance/schedules` - Get maintenance schedules
- `POST /api/iot-advanced/maintenance/train` - Train ML model
- `GET /api/iot-advanced/maintenance/trend` - Get health trend

## Environment Variables

```bash
# MQTT Configuration
MQTT_BROKER_URL=localhost
MQTT_BROKER_PORT=1883
MQTT_CLIENT_ID=smartlms_iot
MQTT_USERNAME=optional_user
MQTT_PASSWORD=optional_pass
```

## MQTT Topics

```
smartlms/iot/sensors/{device_id}    - Sensor data publishing
smartlms/iot/commands/{device_id}   - Device commands
smartlms/iot/alerts/{device_id}     - Alert broadcasting
smartlms/iot/status/{device_id}     - Device status updates
```

## Edge Computing Features

### Rule Types
- **ThresholdAbove**: Trigger when value exceeds threshold
- **ThresholdBelow**: Trigger when value falls below threshold
- **RateOfChange**: Detect rapid changes
- **Anomaly**: Statistical anomaly detection
- **Pattern**: Pattern matching

### Actions
- **Alert**: Generate alert notification
- **Command**: Send device command
- **Filter**: Filter out data at edge
- **Aggregate**: Aggregate before forwarding
- **Forward**: Send to cloud unchanged

## Predictive Maintenance

### Health Metrics Tracked
- Uptime hours
- Error count (24h)
- Average temperature
- Temperature variance
- Vibration level
- Power consumption
- Days since last maintenance
- Device age

### Prediction Statuses
- **Healthy**: No action needed (90+ days to failure)
- **NeedsMaintenance**: Schedule within 7 days
- **Critical**: Immediate attention required

### ML Model
- Decision Tree classifier
- Automatically trains on historical data
- Falls back to heuristic rules when untrained
- Provides confidence scores

## Usage Examples

### Register Edge Node
```bash
curl -X POST http://localhost:8000/api/iot-advanced/edge/nodes \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "building_a_edge",
    "location": "Building A - Floor 1",
    "max_buffer_size": 1000,
    "processing_interval_ms": 1000,
    "enabled_rules": ["temp_alert", "vibration_monitor"]
  }'
```

### Add Edge Rule
```bash
curl -X POST http://localhost:8000/api/iot-advanced/edge/rules \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "building_a_edge",
    "rule_id": "temp_alert",
    "rule_type": "threshold_above",
    "threshold": 75.0,
    "action": "alert"
  }'
```

### Analyze Device Health
```bash
curl -X POST http://localhost:8000/api/iot-advanced/maintenance/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "550e8400-e29b-41d4-a716-446655440000",
    "uptime_hours": 720,
    "error_count_24h": 15,
    "avg_temperature": 82.5,
    "temperature_variance": 5.2,
    "vibration_level": 0.75,
    "power_consumption": 245.8,
    "last_maintenance_days_ago": 95,
    "age_days": 730
  }'
```

### Get Maintenance Schedules
```bash
curl http://localhost:8000/api/iot-advanced/maintenance/schedules
```

## Architecture

```
┌─────────────────┐     MQTT      ┌──────────────────┐
│   IoT Devices   │ ◄──────────► │  MQTT Broker     │
└─────────────────┘               └────────┬─────────┘
                                           │
                          ┌────────────────┼────────────────┐
                          │                │                │
                          ▼                ▼                ▼
                   ┌─────────────┐ ┌─────────────┐ ┌──────────────┐
                   │ Edge Nodes  │ │   Cloud     │ │  Predictive  │
                   │ (Local Proc)│ │  Processor  │ │ Maintenance  │
                   └─────────────┘ └─────────────┘ └──────────────┘
```

## Benefits

1. **Reduced Latency**: Edge processing eliminates round-trip to cloud
2. **Bandwidth Optimization**: Only send aggregated/important data
3. **Proactive Maintenance**: Predict failures before they occur
4. **Real-time Response**: Instant alerts and actions via MQTT
5. **Scalability**: Distributed processing across edge nodes

## Next Steps

1. Install Rust and run `cargo build`
2. Set up MQTT broker (Mosquitto recommended)
3. Configure environment variables
4. Run database migrations
5. Start the server and test endpoints

## Testing

Once Rust is installed:
```bash
cd smartlms-backend
cargo build
cargo run
```

Then test with the curl examples above.
