-- Phase 18: IoT & Smart Campus Integration
-- Provides infrastructure for connecting physical devices to the LMS

-- 1. Device Registry & Management
CREATE TABLE iot_device_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL, -- e.g., 'Attendance Beacon', 'Env Sensor', 'Smart Lock'
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    protocol VARCHAR(50) DEFAULT 'MQTT', -- MQTT, HTTP, CoAP
    data_schema JSONB NOT NULL, -- Expected payload structure
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE iot_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id VARCHAR(100) UNIQUE NOT NULL, -- Hardware serial/UID
    device_type_id UUID REFERENCES iot_device_types(id),
    name VARCHAR(100),
    location_id UUID REFERENCES locations(id), -- Physical location
    room_id UUID REFERENCES rooms(id), -- Specific classroom/lab
    status VARCHAR(50) DEFAULT 'offline', -- online, offline, maintenance, error
    last_seen TIMESTAMPTZ,
    firmware_version VARCHAR(50),
    battery_level INT, -- 0-100 if applicable
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_iot_devices_status ON iot_devices(status);
CREATE INDEX idx_iot_devices_location ON iot_devices(location_id);

CREATE TABLE iot_device_auth (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID REFERENCES iot_devices(id) ON DELETE CASCADE,
    auth_token VARCHAR(255) UNIQUE NOT NULL, -- JWT or API Key for device
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_iot_auth_token ON iot_device_auth(auth_token);

-- 2. Telemetry Data Storage (Time-series optimized)
CREATE TABLE iot_telemetry (
    id BIGSERIAL PRIMARY KEY, -- BigSerial for high volume
    device_id UUID REFERENCES iot_devices(id),
    timestamp TIMESTAMPTZ DEFAULT NOW(),
    metric_name VARCHAR(100) NOT NULL, -- e.g., 'temperature', 'occupancy_count'
    metric_value NUMERIC,
    metric_string VARCHAR(255),
    metadata JSONB,
    processed BOOLEAN DEFAULT FALSE
);
-- Partitioning recommendation for production: BY RANGE (timestamp)
CREATE INDEX idx_iot_telemetry_device_time ON iot_telemetry(device_id, timestamp DESC);
CREATE INDEX idx_iot_telemetry_metric ON iot_telemetry(metric_name, timestamp DESC);

-- 3. Smart Classroom Automation
CREATE TABLE smart_classroom_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID REFERENCES rooms(id),
    name VARCHAR(100) NOT NULL,
    trigger_type VARCHAR(50) NOT NULL, -- 'motion_detected', 'schedule_start', 'manual'
    trigger_config JSONB NOT NULL, -- { "threshold": 1, "device_ids": [...] }
    action_type VARCHAR(50) NOT NULL, -- 'start_recording', 'turn_on_lights', 'adjust_ac'
    action_config JSONB NOT NULL, -- { "device_ids": [...], "params": {...} }
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE smart_classroom_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID REFERENCES rooms(id),
    course_offering_id UUID REFERENCES course_offerings(id),
    started_at TIMESTAMPTZ DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    triggered_by UUID REFERENCES iot_devices(id), -- Which sensor triggered it
    recording_url TEXT,
    status VARCHAR(50) DEFAULT 'active', -- active, completed, error
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 4. Lab Safety & Equipment Monitoring
CREATE TABLE lab_safety_incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lab_id UUID REFERENCES rooms(id),
    device_id UUID REFERENCES iot_devices(id),
    incident_type VARCHAR(100) NOT NULL, -- 'gas_leak', 'high_temp', 'unauthorized_access'
    severity VARCHAR(20) DEFAULT 'medium', -- low, medium, high, critical
    description TEXT,
    detected_at TIMESTAMPTZ DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id),
    notification_sent BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE lab_equipment_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID REFERENCES iot_devices(id),
    user_id UUID REFERENCES users(id), -- Who checked it out/used it
    start_time TIMESTAMPTZ DEFAULT NOW(),
    end_time TIMESTAMPTZ,
    duration_minutes INT GENERATED ALWAYS AS (EXTRACT(EPOCH FROM (end_time - start_time)) / 60) STORED,
    status VARCHAR(50) DEFAULT 'in_use', -- in_use, available, maintenance
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 5. Digital Signage & Notifications
CREATE TABLE digital_signage_screens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screen_id VARCHAR(100) UNIQUE NOT NULL,
    location_id UUID REFERENCES locations(id),
    resolution VARCHAR(20) DEFAULT '1920x1080',
    orientation VARCHAR(20) DEFAULT 'landscape',
    current_content_id UUID,
    status VARCHAR(50) DEFAULT 'online',
    last_heartbeat TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE digital_signage_content (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL,
    content_type VARCHAR(50) DEFAULT 'html', -- html, image, video, json_playlist
    content_data TEXT, -- HTML content or URL to media
    schedule_start TIMESTAMPTZ,
    schedule_end TIMESTAMPTZ,
    priority INT DEFAULT 1, -- Higher = more important (e.g., emergency alerts)
    target_locations JSONB, -- Array of location_ids or 'all'
    is_active BOOLEAN DEFAULT TRUE,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 6. Energy Management
CREATE TABLE energy_metrics (
    id BIGSERIAL PRIMARY KEY,
    building_id UUID REFERENCES locations(id),
    timestamp TIMESTAMPTZ DEFAULT NOW(),
    kwh_consumed NUMERIC,
    solar_generated NUMERIC,
    hvac_load NUMERIC,
    lighting_load NUMERIC,
    cost_estimate NUMERIC,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_energy_metrics_building_time ON energy_metrics(building_id, timestamp DESC);

-- Seed Data: Device Types
INSERT INTO iot_device_types (name, protocol, data_schema) VALUES
('Attendance Beacon', 'MQTT', '{"type": "object", "properties": {"mac_address": {"type": "string"}, "rssi": {"type": "integer"}}}'),
('Environmental Sensor', 'HTTP', '{"type": "object", "properties": {"temp": {"type": "number"}, "humidity": {"type": "number"}, "co2": {"type": "integer"}}}'),
('Smart Lock', 'MQTT', '{"type": "object", "properties": {"state": {"type": "string", "enum": ["locked", "unlocked"]}, "user_id": {"type": "string"}}}'),
('Occupancy Camera', 'HTTP', '{"type": "object", "properties": {"count": {"type": "integer"}, "confidence": {"type": "number"}}}'),
('Energy Meter', 'ModTCP', '{"type": "object", "properties": {"voltage": {"type": "number"}, "current": {"type": "number"}, "power": {"type": "number"}}}');
