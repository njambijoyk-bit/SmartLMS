-- Phase 14: IoT Integration Database Schema
-- Migration 007: IoT devices, sensor readings, commands, and alerts

-- ==================== IoT DEVICES ====================

CREATE TABLE IF NOT EXISTS iot_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    device_type VARCHAR(50) NOT NULL, -- attendance_scanner, environmental_sensor, smart_lock, motion_sensor, energy_meter, security_camera, smart_display, emergency_button, lab_equipment, generic
    status VARCHAR(20) NOT NULL DEFAULT 'offline', -- online, offline, maintenance, error, disabled
    location VARCHAR(255) NOT NULL,
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    course_id UUID REFERENCES courses(id) ON DELETE SET NULL,
    room_id UUID,
    ip_address INET,
    mac_address VARCHAR(17),
    firmware_version VARCHAR(50),
    last_seen TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_iot_devices_institution ON iot_devices(institution_id);
CREATE INDEX idx_iot_devices_type ON iot_devices(device_type);
CREATE INDEX idx_iot_devices_status ON iot_devices(status);
CREATE INDEX idx_iot_devices_course ON iot_devices(course_id);
CREATE INDEX idx_iot_devices_location ON iot_devices(location);

-- ==================== SENSOR READINGS ====================

CREATE TABLE IF NOT EXISTS sensor_readings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sensor_type VARCHAR(100) NOT NULL, -- temperature, humidity, co2, motion, energy, etc.
    value DOUBLE PRECISION NOT NULL,
    unit VARCHAR(50) NOT NULL,
    metadata JSONB
);

CREATE INDEX idx_sensor_readings_device ON sensor_readings(device_id);
CREATE INDEX idx_sensor_readings_timestamp ON sensor_readings(timestamp DESC);
CREATE INDEX idx_sensor_readings_type ON sensor_readings(sensor_type);
CREATE INDEX idx_sensor_readings_device_time ON sensor_readings(device_id, timestamp DESC);

-- Create hypertable for time-series optimization (if TimescaleDB is available)
-- SELECT create_hypertable('sensor_readings', 'timestamp', if_not_exists => TRUE);

-- ==================== DEVICE COMMANDS ====================

CREATE TABLE IF NOT EXISTS device_commands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
    command_type VARCHAR(100) NOT NULL,
    parameters JSONB NOT NULL DEFAULT '{}',
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, sent, executed, failed, timeout
    result JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    executed_at TIMESTAMPTZ
);

CREATE INDEX idx_device_commands_device ON device_commands(device_id);
CREATE INDEX idx_device_commands_status ON device_commands(status);
CREATE INDEX idx_device_commands_created ON device_commands(created_at DESC);

-- ==================== ALERT THRESHOLDS ====================

CREATE TABLE IF NOT EXISTS alert_thresholds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
    sensor_type VARCHAR(100) NOT NULL,
    min_value DOUBLE PRECISION,
    max_value DOUBLE PRECISION,
    severity VARCHAR(20) NOT NULL, -- low, medium, high, critical
    notification_enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_alert_thresholds_device ON alert_thresholds(device_id);
CREATE INDEX idx_alert_thresholds_sensor ON alert_thresholds(sensor_type);

-- ==================== IoT ALERTS ====================

CREATE TABLE IF NOT EXISTS iot_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
    alert_type VARCHAR(100) NOT NULL,
    severity VARCHAR(20) NOT NULL, -- low, medium, high, critical
    message TEXT NOT NULL,
    is_resolved BOOLEAN DEFAULT false,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_iot_alerts_device ON iot_alerts(device_id);
CREATE INDEX idx_iot_alerts_severity ON iot_alerts(severity);
CREATE INDEX idx_iot_alerts_resolved ON iot_alerts(is_resolved) WHERE is_resolved = false;
CREATE INDEX idx_iot_alerts_created ON iot_alerts(created_at DESC);

-- ==================== DEVICE GROUPS ====================

CREATE TABLE IF NOT EXISTS iot_device_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_iot_device_groups_institution ON iot_device_groups(institution_id);

CREATE TABLE IF NOT EXISTS iot_device_group_members (
    device_group_id UUID NOT NULL REFERENCES iot_device_groups(id) ON DELETE CASCADE,
    device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
    added_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (device_group_id, device_id)
);

CREATE INDEX idx_iot_device_group_members_device ON iot_device_group_members(device_id);

-- ==================== DEVICE MAINTENANCE LOGS ====================

CREATE TABLE IF NOT EXISTS device_maintenance_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
    maintenance_type VARCHAR(100) NOT NULL, -- inspection, repair, firmware_update, calibration, cleaning
    description TEXT,
    performed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    cost DECIMAL(10, 2),
    next_maintenance_date DATE,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_device_maintenance_device ON device_maintenance_logs(device_id);
CREATE INDEX idx_device_maintenance_date ON device_maintenance_logs(created_at DESC);

-- ==================== SAMPLE DATA FOR TESTING ====================

-- Insert sample environmental sensor
INSERT INTO iot_devices (id, name, device_type, status, location, institution_id, ip_address, mac_address, metadata)
VALUES 
    ('00000000-0000-0000-0000-000000000001', 'Classroom A Environmental Sensor', 'environmental_sensor', 'online', 
     'Building 1, Room 101', (SELECT id FROM institutions LIMIT 1), 
     '192.168.1.101', '00:1A:2B:3C:4D:5E', 
     '{"manufacturer": "SmartSense", "model": "ENV-2000", "battery_level": 95}'::jsonb);

-- Insert sample attendance scanner
INSERT INTO iot_devices (id, name, device_type, status, location, institution_id, ip_address, mac_address, metadata)
VALUES 
    ('00000000-0000-0000-0000-000000000002', 'Main Entrance Attendance Scanner', 'attendance_scanner', 'online', 
     'Building 1, Main Entrance', (SELECT id FROM institutions LIMIT 1), 
     '192.168.1.102', '00:1A:2B:3C:4D:5F', 
     '{"manufacturer": "AccessTech", "model": "RFID-Pro", "supported_formats": ["RFID", "NFC"]}'::jsonb);

-- Insert sample alert threshold
INSERT INTO alert_thresholds (device_id, sensor_type, min_value, max_value, severity, notification_enabled)
VALUES 
    ('00000000-0000-0000-0000-000000000001', 'temperature', 18.0, 26.0, 'medium', true),
    ('00000000-0000-0000-0000-000000000001', 'humidity', 30.0, 70.0, 'low', true),
    ('00000000-0000-0000-0000-000000000001', 'co2', NULL, 1000.0, 'high', true);

-- Insert sample sensor readings
INSERT INTO sensor_readings (device_id, sensor_type, value, unit, timestamp)
SELECT 
    '00000000-0000-0000-0000-000000000001',
    sensor,
    val,
    unit,
    NOW() - (interval '1 hour' * generate_series(0, 23))
FROM (
    VALUES 
        ('temperature', 22.5 + (random() * 3 - 1.5), 'celsius'),
        ('humidity', 45.0 + (random() * 10 - 5), 'percent'),
        ('co2', 400 + (random() * 200), 'ppm')
) AS data(sensor, val, unit);

COMMENT ON TABLE iot_devices IS 'IoT devices registered in the SmartLMS system';
COMMENT ON TABLE sensor_readings IS 'Time-series sensor data from IoT devices';
COMMENT ON TABLE device_commands IS 'Commands sent to IoT devices';
COMMENT ON TABLE alert_thresholds IS 'Configurable thresholds for triggering alerts';
COMMENT ON TABLE iot_alerts IS 'Alerts generated by IoT devices when thresholds are exceeded';
COMMENT ON TABLE iot_device_groups IS 'Logical grouping of IoT devices';
COMMENT ON TABLE device_maintenance_logs IS 'Maintenance history for IoT devices';
