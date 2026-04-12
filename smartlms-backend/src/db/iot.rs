//! IoT Database Schema Migrations

use sqlx::{PgPool, Executor};

/// Initialize IoT database tables
pub async fn init_iot_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create IoT devices table
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS iot_devices (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            device_type VARCHAR(100) NOT NULL,
            status VARCHAR(50) NOT NULL DEFAULT 'offline',
            location VARCHAR(255) NOT NULL,
            institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
            course_id UUID REFERENCES courses(id) ON DELETE SET NULL,
            room_id UUID,
            ip_address INET,
            mac_address VARCHAR(17),
            firmware_version VARCHAR(100),
            last_seen TIMESTAMPTZ,
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).await?;

    // Create index on institution_id and status for efficient filtering
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_iot_devices_institution ON iot_devices(institution_id)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_iot_devices_status ON iot_devices(status)"
    ).await?;

    // Create sensor readings table
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS sensor_readings (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
            timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            sensor_type VARCHAR(100) NOT NULL,
            value DOUBLE PRECISION NOT NULL,
            unit VARCHAR(50) NOT NULL,
            metadata JSONB
        )
        "#
    ).await?;

    // Create indexes for efficient querying
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_sensor_readings_device ON sensor_readings(device_id)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_sensor_readings_timestamp ON sensor_readings(timestamp DESC)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_sensor_readings_sensor_type ON sensor_readings(sensor_type)"
    ).await?;
    
    // Create composite index for common queries
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_sensor_readings_device_timestamp ON sensor_readings(device_id, timestamp DESC)"
    ).await?;

    // Create device commands table
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS device_commands (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
            command_type VARCHAR(100) NOT NULL,
            parameters JSONB NOT NULL DEFAULT '{}',
            status VARCHAR(50) NOT NULL DEFAULT 'pending',
            result JSONB,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            executed_at TIMESTAMPTZ,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).await?;

    // Create indexes for commands
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_device_commands_device ON device_commands(device_id)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_device_commands_status ON device_commands(status)"
    ).await?;

    // Create IoT alerts table
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS iot_alerts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
            alert_type VARCHAR(255) NOT NULL,
            severity VARCHAR(50) NOT NULL,
            message TEXT NOT NULL,
            is_resolved BOOLEAN NOT NULL DEFAULT false,
            resolved_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).await?;

    // Create indexes for alerts
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_iot_alerts_device ON iot_alerts(device_id)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_iot_alerts_resolved ON iot_alerts(is_resolved)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_iot_alerts_created ON iot_alerts(created_at DESC)"
    ).await?;

    // Create alert thresholds table
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS alert_thresholds (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id UUID NOT NULL REFERENCES iot_devices(id) ON DELETE CASCADE,
            sensor_type VARCHAR(100) NOT NULL,
            min_value DOUBLE PRECISION,
            max_value DOUBLE PRECISION,
            severity VARCHAR(50) NOT NULL,
            notification_enabled BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).await?;

    // Create indexes for thresholds
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_alert_thresholds_device ON alert_thresholds(device_id)"
    ).await?;
    
    pool.execute(
        "CREATE INDEX IF NOT EXISTS idx_alert_thresholds_sensor ON alert_thresholds(device_id, sensor_type)"
    ).await?;

    Ok(())
}

/// Drop all IoT tables (for testing/cleanup)
pub async fn drop_iot_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    pool.execute("DROP TABLE IF EXISTS alert_thresholds CASCADE").await?;
    pool.execute("DROP TABLE IF EXISTS iot_alerts CASCADE").await?;
    pool.execute("DROP TABLE IF EXISTS device_commands CASCADE").await?;
    pool.execute("DROP TABLE IF EXISTS sensor_readings CASCADE").await?;
    pool.execute("DROP TABLE IF EXISTS iot_devices CASCADE").await?;
    Ok(())
}
