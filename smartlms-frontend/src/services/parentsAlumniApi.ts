// API client for Module 18 (Parents Portal), Module 23 (ID Cards), Module 24 (Alumni Portal)
import api from './api';

export const parentsAPI = {
  // Parent Dashboard
  getDashboard: async () => {
    const response = await api.get('/api/parents/dashboard');
    return response.data;
  },

  // Request parent-student link
  requestLink: async (studentEmail: string, linkageType?: string) => {
    const response = await api.post('/api/parents/link', {
      student_email: studentEmail,
      linkage_type: linkageType,
    });
    return response.status;
  },

  // Approve/revoke parent link (student endpoint)
  approveLink: async (linkId: string, approved: boolean) => {
    const response = await api.post('/api/parents/link/approve', {
      link_id: linkId,
      approved,
    });
    return response.status;
  },

  // Get visibility settings
  getVisibility: async (linkId: string) => {
    const response = await api.get(`/api/parents/visibility/${linkId}`);
    return response.data;
  },

  // Update visibility settings (student endpoint)
  updateVisibility: async (linkId: string, settings: any) => {
    const response = await api.put(`/api/parents/visibility/${linkId}`, settings);
    return response.data;
  },

  // Make fee payment
  makePayment: async (studentId: string, amount: number, paymentMethod: string, mpesaPhone?: string) => {
    const response = await api.post('/api/parents/payment', {
      student_id: studentId,
      amount,
      payment_method: paymentMethod,
      mpesa_phone: mpesaPhone,
    });
    return response.data;
  },
};

export const idCardsAPI = {
  // Get my ID card (student endpoint)
  getMyCard: async () => {
    const response = await api.get('/api/id-cards/my-card');
    return response.data;
  },

  // Issue ID card (admin endpoint)
  issueCard: async (userId: string, cardType?: string, expiryDate?: string) => {
    const response = await api.post('/api/id-cards/issue', {
      user_id: userId,
      card_type: cardType,
      expiry_date: expiryDate,
    });
    return response.data;
  },

  // Verify ID card via QR code
  verifyCard: async (qrCode: string, context?: string) => {
    const response = await api.post('/api/id-cards/verify', {
      qr_code: qrCode,
      verification_context: context,
    });
    return response.data;
  },

  // Update card status (admin endpoint)
  updateCardStatus: async (cardId: string, status: string, reason?: string) => {
    const response = await api.put(`/api/id-cards/${cardId}/status`, {
      status,
      reason,
    });
    return response.status;
  },
};

export const alumniAPI = {
  // Get alumni dashboard
  getDashboard: async () => {
    const response = await api.get('/api/alumni/dashboard');
    return response.data;
  },

  // Update alumni profile
  updateProfile: async (profileData: any) => {
    const response = await api.put('/api/alumni/profile', profileData);
    return response.data;
  },

  // Search alumni directory
  searchDirectory: async (params: {
    q?: string;
    year?: number;
    programme?: string;
    location?: string;
    limit?: number;
  }) => {
    const response = await api.get('/api/alumni/directory', { params });
    return response.data;
  },

  // Post a job
  postJob: async (jobData: any) => {
    const response = await api.post('/api/alumni/jobs', jobData);
    return response.data;
  },

  // Apply to job
  applyToJob: async (jobId: string, coverLetter?: string, resumeUrl?: string) => {
    const response = await api.post('/api/alumni/jobs/apply', {
      job_id: jobId,
      cover_letter: coverLetter,
      resume_url: resumeUrl,
    });
    return response.data;
  },

  // Connect with alumni
  connectWithAlumni: async (alumniId: string, connectionType?: string, message?: string) => {
    const response = await api.post('/api/alumni/connect', {
      alumni_id: alumniId,
      connection_type: connectionType,
      message,
    });
    return response.status;
  },

  // Make donation
  makeDonation: async (amount: number, donationType?: string, fundDesignation?: string, 
                       paymentMethod: string = 'mpesa', anonymous?: boolean, message?: string) => {
    const response = await api.post('/api/alumni/donate', {
      amount,
      donation_type: donationType,
      fund_designation: fundDesignation,
      payment_method: paymentMethod,
      anonymous,
      message,
    });
    return response.data;
  },

  // Download transcript
  downloadTranscript: async () => {
    const response = await api.get('/api/alumni/transcript', { responseType: 'blob' });
    return response.data;
  },
};

// Re-export for convenience
export { api };
