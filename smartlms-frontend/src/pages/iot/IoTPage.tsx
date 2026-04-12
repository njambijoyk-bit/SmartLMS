import React, { useState, useEffect } from 'react';
import { Card } from '../../components/ui/Card';
import { Button } from '../../components/ui/Button';
import { Badge } from '../../components/ui/Badge';

interface IoTDevice {
  id: string;
  device_name: string;
  device_type: string;
  status: 'online' | 'offline' | 'error' | 'maintenance';
  institution_id: string;
  location?: string;
  last_seen?: string;
  firmware_version?: string;
}

interface SensorReading {
  id: string;
  device_id: string;
  sensor_type: string;
  value: number;
  unit: string;
  timestamp: string;
}

interface IotAlert {
  id: string;
  device_id: string;
  alert_type: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  message: string;
  created_at: string;
  acknowledged: boolean;
}

const IoTPage: React.FC = () => {
  const [devices, setDevices] = useState<IoTDevice[]>([]);
  const [readings, setReadings] = useState<SensorReading[]>([]);
  const [alerts, setAlerts] = useState<IotAlert[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedDeviceType, setSelectedDeviceType] = useState('all');
  const [showAddDeviceModal, setShowAddDeviceModal] = useState(false);

  useEffect(() => {
    loadIoTData();
  }, []);

  const loadIoTData = async () => {
    try {
      setLoading(true);
      // TODO: Replace with actual API calls when available
      // const [devicesData, readingsData, alertsData] = await Promise.all([
      //   iotAPI.devices.list(institutionId),
      //   iotAPI.readings.list({ limit: 50 }),
      //   iotAPI.alerts.list(),
      // ]);
      // setDevices(devicesData);
      // setReadings(readingsData);
      // setAlerts(alertsData);
      
      // Mock data for demonstration
      setDevices([
        {
          id: '1',
          device_name: 'Smart Thermostat - Room 101',
          device_type: 'thermostat',
          status: 'online',
          institution_id: 'inst-1',
          location: 'Building A, Room 101',
          last_seen: new Date().toISOString(),
          firmware_version: '2.1.0',
        },
        {
          id: '2',
          device_name: 'Motion Sensor - Hallway',
          device_type: 'motion_sensor',
          status: 'online',
          institution_id: 'inst-1',
          location: 'Building A, Hallway',
          last_seen: new Date().toISOString(),
          firmware_version: '1.5.2',
        },
        {
          id: '3',
          device_name: 'Air Quality Monitor - Lab',
          device_type: 'air_quality',
          status: 'error',
          institution_id: 'inst-1',
          location: 'Building B, Lab 3',
          last_seen: new Date(Date.now() - 3600000).toISOString(),
          firmware_version: '1.2.0',
        },
      ]);
      
      setReadings([
        { id: '1', device_id: '1', sensor_type: 'temperature', value: 22.5, unit: '°C', timestamp: new Date().toISOString() },
        { id: '2', device_id: '1', sensor_type: 'humidity', value: 45, unit: '%', timestamp: new Date().toISOString() },
        { id: '3', device_id: '2', sensor_type: 'motion', value: 1, unit: 'detected', timestamp: new Date().toISOString() },
        { id: '4', device_id: '3', sensor_type: 'co2', value: 450, unit: 'ppm', timestamp: new Date(Date.now() - 3600000).toISOString() },
      ]);
      
      setAlerts([
        {
          id: '1',
          device_id: '3',
          alert_type: 'connection_lost',
          severity: 'high',
          message: 'Device has not reported in 1 hour',
          created_at: new Date(Date.now() - 3600000).toISOString(),
          acknowledged: false,
        },
      ]);
    } catch (error) {
      console.error('Failed to load IoT data:', error);
    } finally {
      setLoading(false);
    }
  };

  const getStatusBadgeVariant = (status: string) => {
    switch (status) {
      case 'online':
        return 'success' as const;
      case 'offline':
        return 'secondary' as const;
      case 'error':
        return 'danger' as const;
      case 'maintenance':
        return 'warning' as const;
      default:
        return 'secondary' as const;
    }
  };

  const getSeverityBadgeVariant = (severity: string) => {
    switch (severity) {
      case 'low':
        return 'secondary' as const;
      case 'medium':
        return 'warning' as const;
      case 'high':
        return 'danger' as const;
      case 'critical':
        return 'danger' as const;
      default:
        return 'secondary' as const;
    }
  };

  const filteredDevices = selectedDeviceType === 'all' 
    ? devices 
    : devices.filter(d => d.device_type === selectedDeviceType);

  const deviceTypes = Array.from(new Set(devices.map(d => d.device_type)));

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">IoT Devices</h1>
          <p className="text-gray-600 mt-1">Monitor and manage connected devices</p>
        </div>
        <Button
          onClick={() => setShowAddDeviceModal(true)}
          className="bg-blue-600 hover:bg-blue-700"
        >
          Add Device
        </Button>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-green-100 rounded-md p-3">
              <svg className="h-6 w-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Online</p>
              <p className="text-2xl font-semibold text-gray-900">
                {devices.filter(d => d.status === 'online').length}
              </p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-gray-100 rounded-md p-3">
              <svg className="h-6 w-6 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Offline</p>
              <p className="text-2xl font-semibold text-gray-900">
                {devices.filter(d => d.status === 'offline').length}
              </p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-red-100 rounded-md p-3">
              <svg className="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Errors</p>
              <p className="text-2xl font-semibold text-gray-900">
                {devices.filter(d => d.status === 'error').length}
              </p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-yellow-100 rounded-md p-3">
              <svg className="h-6 w-6 text-yellow-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Active Alerts</p>
              <p className="text-2xl font-semibold text-gray-900">
                {alerts.filter(a => !a.acknowledged).length}
              </p>
            </div>
          </div>
        </Card>
      </div>

      {/* Device Filters */}
      <Card>
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold">Devices</h3>
          <select
            value={selectedDeviceType}
            onChange={(e) => setSelectedDeviceType(e.target.value)}
            className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="all">All Types</option>
            {deviceTypes.map(type => (
              <option key={type} value={type}>{type}</option>
            ))}
          </select>
        </div>
        {loading && devices.length === 0 ? (
          <div className="text-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
          </div>
        ) : filteredDevices.length === 0 ? (
          <div className="text-center py-8">
            <p className="text-gray-500">No devices found</p>
            <Button
              variant="secondary"
              onClick={() => setShowAddDeviceModal(true)}
              className="mt-4"
            >
              Add Your First Device
            </Button>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Device
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Type
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Location
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Last Seen
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {filteredDevices.map((device) => (
                  <tr key={device.id}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-medium text-gray-900">{device.device_name}</div>
                      <div className="text-sm text-gray-500">v{device.firmware_version}</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm text-gray-700 capitalize">{device.device_type.replace('_', ' ')}</span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm text-gray-500">{device.location || '-'}</span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <Badge variant={getStatusBadgeVariant(device.status)}>
                        {device.status}
                      </Badge>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {device.last_seen ? new Date(device.last_seen).toLocaleString() : 'Never'}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <button className="text-blue-600 hover:text-blue-900 mr-3">View</button>
                      <button className="text-gray-600 hover:text-gray-900">Configure</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </Card>

      {/* Recent Readings */}
      <Card>
        <h3 className="text-lg font-semibold mb-4">Recent Sensor Readings</h3>
        {readings.length === 0 ? (
          <p className="text-gray-500 text-center py-4">No recent readings</p>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {readings.map((reading) => (
              <div key={reading.id} className="bg-gray-50 rounded-lg p-4">
                <p className="text-sm text-gray-500">{reading.sensor_type}</p>
                <p className="text-2xl font-bold text-gray-900">
                  {reading.value} <span className="text-sm font-normal">{reading.unit}</span>
                </p>
                <p className="text-xs text-gray-400 mt-2">
                  {new Date(reading.timestamp).toLocaleString()}
                </p>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* Active Alerts */}
      {alerts.filter(a => !a.acknowledged).length > 0 && (
        <Card>
          <h3 className="text-lg font-semibold mb-4">Active Alerts</h3>
          <div className="space-y-3">
            {alerts.filter(a => !a.acknowledged).map((alert) => (
              <div
                key={alert.id}
                className={`p-4 rounded-lg border-l-4 ${
                  alert.severity === 'critical' ? 'bg-red-50 border-red-500' :
                  alert.severity === 'high' ? 'bg-orange-50 border-orange-500' :
                  alert.severity === 'medium' ? 'bg-yellow-50 border-yellow-500' :
                  'bg-gray-50 border-gray-500'
                }`}
              >
                <div className="flex justify-between items-start">
                  <div>
                    <p className="font-semibold text-gray-900">{alert.alert_type.replace('_', ' ').toUpperCase()}</p>
                    <p className="text-sm text-gray-700 mt-1">{alert.message}</p>
                    <p className="text-xs text-gray-500 mt-2">
                      Device: {alert.device_id} • {new Date(alert.created_at).toLocaleString()}
                    </p>
                  </div>
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={() => {
                      // TODO: Acknowledge alert
                      setAlerts(alerts.map(a => a.id === alert.id ? {...a, acknowledged: true} : a));
                    }}
                  >
                    Acknowledge
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}

      {/* Add Device Modal */}
      {showAddDeviceModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <Card className="w-full max-w-md p-6">
            <h3 className="text-lg font-semibold mb-4">Add New IoT Device</h3>
            <form className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Device Name
                </label>
                <input
                  type="text"
                  required
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="e.g., Smart Thermostat - Room 101"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Device Type
                </label>
                <select className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500">
                  <option value="thermostat">Thermostat</option>
                  <option value="motion_sensor">Motion Sensor</option>
                  <option value="air_quality">Air Quality Monitor</option>
                  <option value="smart_lock">Smart Lock</option>
                  <option value="camera">Security Camera</option>
                  <option value="other">Other</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Location
                </label>
                <input
                  type="text"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="e.g., Building A, Room 101"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Device ID / MAC Address
                </label>
                <input
                  type="text"
                  required
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="e.g., 00:1B:44:11:3A:B7"
                />
              </div>
              <div className="flex justify-end space-x-2 pt-4">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => setShowAddDeviceModal(false)}
                >
                  Cancel
                </Button>
                <Button type="submit" className="bg-blue-600 hover:bg-blue-700">
                  Add Device
                </Button>
              </div>
            </form>
          </Card>
        </div>
      )}
    </div>
  );
};

export default IoTPage;
