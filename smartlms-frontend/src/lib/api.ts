import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8000';

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor for error handling
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Handle unauthorized - redirect to login
      localStorage.removeItem('auth_token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export default api;

// Phase 16 & 17 API Methods
export const developerAPI = {
  // VPAT Reports
  vpat: {
    generate: async (productId: string, wcagLevel: 'A' | 'AA' | 'AAA') => {
      const response = await api.post(`/api/v1/vpat/generate`, { productId, wcagLevel });
      return response.data;
    },
    getById: async (id: string) => {
      const response = await api.get(`/api/v1/vpat/${id}`);
      return response.data;
    },
    list: async () => {
      const response = await api.get(`/api/v1/vpat`);
      return response.data;
    },
    updateCriterion: async (reportId: string, criterionId: string, data: any) => {
      const response = await api.put(`/api/v1/vpat/${reportId}/criteria/${criterionId}`, data);
      return response.data;
    },
    exportPDF: async (id: string) => {
      const response = await api.get(`/api/v1/vpat/${id}/export/pdf`, { responseType: 'blob' });
      return response.data;
    },
  },

  // OAuth Applications
  oauth: {
    createApplication: async (data: any) => {
      const response = await api.post(`/api/v1/oauth/applications`, data);
      return response.data;
    },
    getApplications: async () => {
      const response = await api.get(`/api/v1/oauth/applications`);
      return response.data;
    },
    getApplication: async (id: string) => {
      const response = await api.get(`/api/v1/oauth/applications/${id}`);
      return response.data;
    },
    updateApplication: async (id: string, data: any) => {
      const response = await api.put(`/api/v1/oauth/applications/${id}`, data);
      return response.data;
    },
    deleteApplication: async (id: string) => {
      const response = await api.delete(`/api/v1/oauth/applications/${id}`);
      return response.data;
    },
    regenerateSecret: async (id: string) => {
      const response = await api.post(`/api/v1/oauth/applications/${id}/regenerate-secret`);
      return response.data;
    },
    getTokens: async (applicationId: string) => {
      const response = await api.get(`/api/v1/oauth/applications/${applicationId}/tokens`);
      return response.data;
    },
    revokeToken: async (tokenId: string) => {
      const response = await api.post(`/api/v1/oauth/tokens/${tokenId}/revoke`);
      return response.data;
    },
  },

  // Marketplace
  marketplace: {
    getApps: async (category?: string) => {
      const params = category ? { category } : {};
      const response = await api.get(`/api/v1/marketplace/apps`, { params });
      return response.data;
    },
    getApp: async (id: string) => {
      const response = await api.get(`/api/v1/marketplace/apps/${id}`);
      return response.data;
    },
    submitApp: async (data: any) => {
      const response = await api.post(`/api/v1/marketplace/apps`, data);
      return response.data;
    },
    updateApp: async (id: string, data: any) => {
      const response = await api.put(`/api/v1/marketplace/apps/${id}`, data);
      return response.data;
    },
    installApp: async (id: string) => {
      const response = await api.post(`/api/v1/marketplace/apps/${id}/install`);
      return response.data;
    },
    getInstalledApps: async () => {
      const response = await api.get(`/api/v1/marketplace/installed`);
      return response.data;
    },
  },

  // SDK Generator
  sdk: {
    generate: async (config: any) => {
      const response = await api.post(`/api/v1/sdk/generate`, config);
      return response.data;
    },
    getPackages: async () => {
      const response = await api.get(`/api/v1/sdk/packages`);
      return response.data;
    },
    downloadPackage: async (id: string) => {
      const response = await api.get(`/api/v1/sdk/packages/${id}/download`, { responseType: 'blob' });
      return response.data;
    },
  },

  // API Analytics
  analytics: {
    getDashboard: async (timeRange: string = '7d') => {
      const response = await api.get(`/api/v1/analytics/dashboard`, { params: { timeRange } });
      return response.data;
    },
    getEndpointStats: async (timeRange: string = '7d') => {
      const response = await api.get(`/api/v1/analytics/endpoints`, { params: { timeRange } });
      return response.data;
    },
    getUserStats: async (timeRange: string = '7d') => {
      const response = await api.get(`/api/v1/analytics/users`, { params: { timeRange } });
      return response.data;
    },
    getQuota: async () => {
      const response = await api.get(`/api/v1/analytics/quota`);
      return response.data;
    },
  },

  // LMS Migration
  migration: {
    startMoodle: async (config: any) => {
      const response = await api.post(`/api/v1/migration/moodle`, config);
      return response.data;
    },
    startCanvas: async (config: any) => {
      const response = await api.post(`/api/v1/migration/canvas`, config);
      return response.data;
    },
    getStatus: async (id: string) => {
      const response = await api.get(`/api/v1/migration/${id}/status`);
      return response.data;
    },
    listMigrations: async () => {
      const response = await api.get(`/api/v1/migration`);
      return response.data;
    },
  },

  // SOC 2 Compliance
  compliance: {
    getDashboard: async () => {
      const response = await api.get(`/api/v1/compliance/dashboard`);
      return response.data;
    },
    getControls: async (category?: string) => {
      const params = category ? { category } : {};
      const response = await api.get(`/api/v1/compliance/controls`, { params });
      return response.data;
    },
    updateControl: async (id: string, data: any) => {
      const response = await api.put(`/api/v1/compliance/controls/${id}`, data);
      return response.data;
    },
    addEvidence: async (controlId: string, evidence: any) => {
      const response = await api.post(`/api/v1/compliance/controls/${controlId}/evidence`, evidence);
      return response.data;
    },
    getAudits: async () => {
      const response = await api.get(`/api/v1/compliance/audits`);
      return response.data;
    },
    createAudit: async (data: any) => {
      const response = await api.post(`/api/v1/compliance/audits`, data);
      return response.data;
    },
  },

  // Phase 18: Blockchain Certificates
  blockchain: {
    // Get all certificates for user
    getCertificates: async () => {
      const response = await api.get(`/api/v1/blockchain/certificates`);
      return response.data;
    },
    
    // Mint a certificate
    mintCertificate: async (certificateId: string, network: string = 'polygon') => {
      const response = await api.post(`/api/v1/blockchain/certificates/${certificateId}/mint`, {
        network,
      });
      return response.data;
    },
    
    // Batch mint certificates
    batchMint: async (institutionId: string, certificateIds: string[], network: string, priority: string) => {
      const response = await api.post(`/api/v1/blockchain/certificates/batch-mint`, {
        institution_id: institutionId,
        certificate_ids: certificateIds,
        network,
        priority,
      });
      return response.data;
    },
    
    // Get batch status
    getBatchStatus: async (batchId: string) => {
      const response = await api.get(`/api/v1/blockchain/certificates/batch/${batchId}/status`);
      return response.data;
    },
    
    // Verify certificate
    verifyCertificate: async (data: { certificate_hash?: string; token_id?: string; transaction_hash?: string }) => {
      const response = await api.post(`/api/v1/blockchain/verify`, data);
      return response.data;
    },
    
    // Get certificate proof
    getProof: async (certificateId: string) => {
      const response = await api.get(`/api/v1/blockchain/certificates/${certificateId}/proof`);
      return response.data;
    },
    
    // QR code verification
    qrVerify: async (code: string) => {
      const response = await api.get(`/api/v1/blockchain/verify/qr/${code}`);
      return response.data;
    },
    
    // Generate QR code for certificate
    generateQR: async (certificateId: string) => {
      const response = await api.get(`/api/v1/blockchain/certificates/${certificateId}/qr`);
      return response.data;
    },
    
    // Connect wallet
    connectWallet: async (userId: string, walletAddress: string, signature: string, message: string) => {
      const response = await api.post(`/api/v1/blockchain/wallet/connect`, {
        user_id: userId,
        wallet_address: walletAddress,
        signature,
        message,
      });
      return response.data;
    },
    
    // Disconnect wallet
    disconnectWallet: async (userId: string) => {
      const response = await api.post(`/api/v1/blockchain/wallet/${userId}/disconnect`);
      return response.data;
    },
    
    // Withdraw certificate to wallet
    withdrawCertificate: async (userId: string, certificateId: string, walletAddress: string) => {
      const response = await api.post(`/api/v1/blockchain/wallet/withdraw`, {
        user_id: userId,
        certificate_id: certificateId,
        wallet_address: walletAddress,
      });
      return response.data;
    },
    
    // Get gas prices
    getGasPrices: async () => {
      const response = await api.get(`/api/v1/blockchain/gas/prices`);
      return response.data;
    },
    
    // Estimate gas
    estimateGas: async (certificateId: string, network: string) => {
      const response = await api.post(`/api/v1/blockchain/gas/estimate`, {
        certificate_id: certificateId,
        network,
      });
      return response.data;
    },
    
    // Public verification
    publicVerify: async (identifier: string) => {
      const response = await api.get(`/api/v1/blockchain/public/${identifier}`);
      return response.data;
    },
  },
};

export const blockchainAPI = developerAPI.blockchain;
