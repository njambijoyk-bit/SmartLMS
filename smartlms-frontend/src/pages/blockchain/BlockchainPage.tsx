import React, { useState, useEffect } from 'react';
import { Card } from '../../components/ui/Card';
import { Button } from '../../components/ui/Button';
import { Badge } from '../../components/ui/Badge';
import { blockchainAPI } from '../../lib/api';

interface BlockchainCertificate {
  id: string;
  certificate_id: string;
  user_id: string;
  course_id: string;
  institution_id: string;
  token_id?: string;
  transaction_hash?: string;
  contract_address?: string;
  network: string;
  mint_status: 'pending' | 'minting' | 'minted' | 'failed' | 'revoked';
  ipfs_hash?: string;
  minted_at?: string;
  created_at: string;
}

interface VerificationResult {
  is_valid: boolean;
  certificate_info?: {
    certificate_name: string;
    recipient_name: string;
    institution_name: string;
    issue_date: string;
    credential_type: string;
    grade?: string;
    honors?: string;
  };
  blockchain_proof?: {
    network: string;
    contract_address: string;
    token_id: string;
    transaction_hash: string;
    block_number?: number;
    explorer_url: string;
  };
  verification_timestamp: string;
  error?: string;
}

const BlockchainPage: React.FC = () => {
  const [certificates, setCertificates] = useState<BlockchainCertificate[]>([]);
  const [loading, setLoading] = useState(false);
  const [showMintModal, setShowMintModal] = useState(false);
  const [showVerifyModal, setShowVerifyModal] = useState(false);
  const [verificationQuery, setVerificationQuery] = useState('');
  const [verificationResult, setVerificationResult] = useState<VerificationResult | null>(null);
  const [selectedNetwork, setSelectedNetwork] = useState('polygon');
  const [certificateId, setCertificateId] = useState('');

  useEffect(() => {
    loadCertificates();
  }, []);

  const loadCertificates = async () => {
    try {
      setLoading(true);
      const data = await blockchainAPI.getCertificates();
      setCertificates(data);
    } catch (error) {
      console.error('Failed to load certificates:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleMintCertificate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      await blockchainAPI.mintCertificate(certificateId, selectedNetwork);
      alert('Certificate minting initiated!');
      setShowMintModal(false);
      loadCertificates();
    } catch (error) {
      console.error('Failed to mint certificate:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleVerifyCertificate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      const result = await blockchainAPI.verifyCertificate({
        certificate_hash: verificationQuery,
      });
      setVerificationResult(result);
      setShowVerifyModal(true);
    } catch (error) {
      console.error('Failed to verify certificate:', error);
    } finally {
      setLoading(false);
    }
  };

  const getExplorerUrl = (network: string, txHash?: string) => {
    const explorers: Record<string, string> = {
      polygon: 'https://polygonscan.com/tx/',
      ethereum: 'https://etherscan.io/tx/',
      mumbai: 'https://mumbai.polygonscan.com/tx/',
    };
    return txHash ? `${explorers[network] || explorers.polygon}${txHash}` : '#';
  };

  const getStatusBadgeVariant = (status: string) => {
    switch (status) {
      case 'minted':
        return 'success' as const;
      case 'minting':
        return 'info' as const;
      case 'failed':
        return 'danger' as const;
      case 'revoked':
        return 'default' as const;
      default:
        return 'default' as const;
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Blockchain Certificates</h1>
          <p className="text-gray-600 mt-1">NFT-based credentials on the blockchain</p>
        </div>
        <div className="flex gap-3">
          <Button
            variant="secondary"
            onClick={() => setShowVerifyModal(true)}
          >
            Verify Certificate
          </Button>
          <Button
            onClick={() => setShowMintModal(true)}
            className="bg-blue-600 hover:bg-blue-700"
          >
            Mint Certificate
          </Button>
        </div>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-blue-100 rounded-md p-3">
              <svg className="h-6 w-6 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Total Minted</p>
              <p className="text-2xl font-semibold text-gray-900">
                {certificates.filter(c => c.mint_status === 'minted').length}
              </p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-yellow-100 rounded-md p-3">
              <svg className="h-6 w-6 text-yellow-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Pending</p>
              <p className="text-2xl font-semibold text-gray-900">
                {certificates.filter(c => c.mint_status === 'pending' || c.mint_status === 'minting').length}
              </p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-green-100 rounded-md p-3">
              <svg className="h-6 w-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Verified</p>
              <p className="text-2xl font-semibold text-gray-900">{certificates.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0 bg-purple-100 rounded-md p-3">
              <svg className="h-6 w-6 text-purple-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500">Network</p>
              <p className="text-2xl font-semibold text-gray-900 capitalize">{selectedNetwork}</p>
            </div>
          </div>
        </Card>
      </div>

      {/* Certificates List */}
      <Card>
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-lg font-semibold">Blockchain Certificates</h3>
        </div>
        {loading && certificates.length === 0 ? (
          <div className="text-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
          </div>
        ) : certificates.length === 0 ? (
          <div className="text-center py-8">
            <p className="text-gray-500">No blockchain certificates yet</p>
            <Button
              variant="secondary"
              onClick={() => setShowMintModal(true)}
              className="mt-4"
            >
              Mint Your First Certificate
            </Button>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Certificate
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Token ID
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Network
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Minted At
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {certificates.map((cert) => (
                  <tr key={cert.id}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-medium text-gray-900">{cert.certificate_id}</div>
                      <div className="text-sm text-gray-500">{cert.user_id}</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      {cert.token_id ? (
                        <code className="text-xs bg-gray-100 px-2 py-1 rounded">{cert.token_id}</code>
                      ) : (
                        <span className="text-gray-400">-</span>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="text-sm text-gray-700 capitalize">{cert.network}</span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <Badge variant={getStatusBadgeVariant(cert.mint_status)}>
                        {cert.mint_status}
                      </Badge>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {cert.minted_at ? new Date(cert.minted_at).toLocaleDateString() : '-'}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      {cert.transaction_hash && (
                        <a
                          href={getExplorerUrl(cert.network, cert.transaction_hash)}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-blue-600 hover:text-blue-900"
                        >
                          View on Explorer
                        </a>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </Card>

      {/* Mint Modal */}
      {showMintModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <Card className="w-full max-w-md p-6">
            <h3 className="text-lg font-semibold mb-4">Mint Certificate as NFT</h3>
            <form onSubmit={handleMintCertificate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Certificate ID
                </label>
                <input
                  type="text"
                  value={certificateId}
                  onChange={(e) => setCertificateId(e.target.value)}
                  required
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter certificate ID"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Blockchain Network
                </label>
                <select
                  value={selectedNetwork}
                  onChange={(e) => setSelectedNetwork(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="polygon">Polygon (Recommended)</option>
                  <option value="ethereum">Ethereum Mainnet</option>
                  <option value="mumbai">Polygon Mumbai (Testnet)</option>
                </select>
              </div>
              <div className="bg-blue-50 p-4 rounded-md">
                <p className="text-sm text-blue-800">
                  <strong>Note:</strong> Minting requires gas fees. Polygon network is recommended for lower costs.
                </p>
              </div>
              <div className="flex justify-end space-x-2 pt-4">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => setShowMintModal(false)}
                >
                  Cancel
                </Button>
                <Button type="submit" className="bg-blue-600 hover:bg-blue-700">
                  Mint Certificate
                </Button>
              </div>
            </form>
          </Card>
        </div>
      )}

      {/* Verify Modal */}
      {showVerifyModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <Card className="w-full max-w-lg p-6">
            <h3 className="text-lg font-semibold mb-4">Verify Blockchain Certificate</h3>
            <form onSubmit={handleVerifyCertificate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Certificate Hash / Token ID / Transaction Hash
                </label>
                <input
                  type="text"
                  value={verificationQuery}
                  onChange={(e) => setVerificationQuery(e.target.value)}
                  required
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter hash or token ID"
                />
              </div>
              <div className="flex justify-end space-x-2 pt-4">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => setShowVerifyModal(false)}
                >
                  Close
                </Button>
                <Button type="submit" className="bg-blue-600 hover:bg-blue-700">
                  Verify
                </Button>
              </div>
            </form>
            {verificationResult && (
              <div className="mt-4 pt-4 border-t">
                <div className={`p-4 rounded-md ${verificationResult.is_valid ? 'bg-green-50' : 'bg-red-50'}`}>
                  <p className={`font-semibold ${verificationResult.is_valid ? 'text-green-800' : 'text-red-800'}`}>
                    {verificationResult.is_valid ? '✓ Valid Certificate' : '✗ Invalid Certificate'}
                  </p>
                  {verificationResult.certificate_info && (
                    <div className="mt-2 text-sm text-gray-700">
                      <p><strong>Name:</strong> {verificationResult.certificate_info.certificate_name}</p>
                      <p><strong>Recipient:</strong> {verificationResult.certificate_info.recipient_name}</p>
                      <p><strong>Institution:</strong> {verificationResult.certificate_info.institution_name}</p>
                      <p><strong>Issued:</strong> {new Date(verificationResult.certificate_info.issue_date).toLocaleDateString()}</p>
                    </div>
                  )}
                  {verificationResult.blockchain_proof && (
                    <div className="mt-2 text-sm text-gray-700">
                      <p><strong>Network:</strong> {verificationResult.blockchain_proof.network}</p>
                      <p><strong>Token ID:</strong> {verificationResult.blockchain_proof.token_id}</p>
                      <a
                        href={verificationResult.blockchain_proof.explorer_url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-blue-600 hover:underline"
                      >
                        View on Blockchain Explorer →
                      </a>
                    </div>
                  )}
                </div>
              </div>
            )}
          </Card>
        </div>
      )}
    </div>
  );
};

export default BlockchainPage;
