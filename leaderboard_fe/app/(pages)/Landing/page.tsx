'use client';
import React, { useState, useEffect } from 'react';
import Header from '../../components/Landing/Header';
import Card from '../../components/Landing/card';
import Pagination from '../../components/Landing/Pagination';

const API_BASE_URL = 'http://localhost:3000/api/v1';

interface LeaderboardEntry {
  address: string;
  rank: number;
  buy_count: number;
  sell_count: number;
  total_volume_usd: string; // Backend returns as string due to BigDecimal
  first_trade_at: string | null;
  last_trade_at: string | null;
}

interface ApiResponse {
  address: string;
  buy_count: number;
  sell_count: number;
  total_volume_usd: string;
  first_trade_at: string | null;
  last_trade_at: string | null;
}

const LandingPage = () => {
  const [currentPage, setCurrentPage] = useState(1);
  const [addresses, setAddresses] = useState<LeaderboardEntry[]>([]);
  const [isLoaded, setIsLoaded] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [isSyncing, setIsSyncing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const itemsPerPage = 10;

  const fetchLeaderboardData = async (page: number = 1, pageSize: number = 100) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`${API_BASE_URL}/leaderboard?page=${page}&page_size=${pageSize}`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: ApiResponse[] = await response.json();
      
      // Map the API response to our interface
      const mappedData: LeaderboardEntry[] = data.map((item, index) => ({
        address: item.address,
        rank: index + 1,
        buy_count: item.buy_count || 0,
        sell_count: item.sell_count || 0,
        total_volume_usd: item.total_volume_usd || "0",
        first_trade_at: item.first_trade_at,
        last_trade_at: item.last_trade_at
      }));
      
      setAddresses(mappedData);
      setIsLoaded(true);
    } catch (err) {
      console.error('Error fetching leaderboard data:', err);
      setError('Failed to load leaderboard data. Please ensure database is connected.');
      setIsLoaded(true);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSync = async () => {
    setIsSyncing(true);
    setError(null);
    try {
      const response = await fetch(`${API_BASE_URL}/sync`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });
      
      if (!response.ok) {
        throw new Error(`Sync failed! status: ${response.status}`);
      }
      
      // After successful sync, refresh the leaderboard data
      await fetchLeaderboardData();
    } catch (err) {
      console.error('Error syncing data:', err);
      setError('Failed to sync data. Please ensure database is connected.');
    } finally {
      setIsSyncing(false);
    }
  };

  useEffect(() => {
    fetchLeaderboardData();
  }, []);

  const totalPages = Math.ceil(addresses.length / itemsPerPage);

  const currentAddresses = addresses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  const handlePageChange = (page: number) => {
    setCurrentPage(page);
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  if (isLoading && !isLoaded) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
          <div className="text-white text-xl">Loading leaderboard data...</div>
        </div>
      </div>
    );
  }

  if (error && !isLoaded) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center">
        <div className="text-center">
          <div className="text-red-400 text-xl mb-4">{error}</div>
          <button 
            onClick={() => fetchLeaderboardData()}
            className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded-lg transition-colors"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  const startItem = (currentPage - 1) * itemsPerPage + 1;
  const endItem = Math.min(currentPage * itemsPerPage, addresses.length);

  return (
    <div className="min-h-screen bg-gray-900">
      <Header onSync={handleSync} isSyncing={isSyncing} />
      
      <main className="max-w-4xl mx-auto px-4 py-8">
        {/* Mock data indicator */}
        {/* Error display */}
        {error && (
          <div className="bg-red-900/20 border border-red-500/50 text-red-400 px-4 py-3 rounded-lg mb-6">
            {error}
          </div>
        )}

        {/* Results counter, pagination, and sync button at top */}
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-6">
            <Pagination
              currentPage={currentPage}
              totalPages={totalPages}
              onPageChange={handlePageChange}
            />
            <p className="text-gray-400 text-sm">
              Showing {startItem}-{endItem} of {addresses.length} addresses (Page {currentPage} of {totalPages})
            </p>
          </div>
          
          {/* Sync button */}
          <button
            onClick={handleSync}
            disabled={isSyncing}
            className="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-4 py-2 rounded-lg transition-colors flex items-center space-x-2"
          >
            {isSyncing ? (
              <>
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                <span>Syncing...</span>
              </>
            ) : (
              <>
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
                <span>Sync</span>
              </>
            )}
          </button>
        </div>

        <div className="space-y-4">
          {currentAddresses.map((item) => (
            <Card
              key={item.address}
              address={item.address}
              rank={item.rank}
              tradingVolume={item.total_volume_usd}
              buyCount={item.buy_count}
              sellCount={item.sell_count}
              lastTradeAt={item.last_trade_at}
              firstTradeAt={item.first_trade_at}
            />
          ))}
        </div>

        {addresses.length === 0 && isLoaded && (
          <div className="text-center text-gray-400 py-8">
            <p className="text-lg mb-4">No leaderboard data available.</p>
            <p className="text-sm mb-4">Click the Sync button to load the latest trading data from Uniswap.</p>
            <button
              onClick={handleSync}
              disabled={isSyncing}
              className="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 text-white px-6 py-2 rounded-lg transition-colors"
            >
              {isSyncing ? 'Syncing...' : 'Sync Data'}
            </button>
          </div>
        )}
      </main>
    </div>
  );
};

export default LandingPage;