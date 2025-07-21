'use client';
import React, { useState, useEffect } from 'react';
import Header from '../../components/Landing/Header';
import Card from '../../components/Landing/card';
import Pagination from '../../components/Landing/Pagination';

// Dummy data for addresses with realistic trading data
const generateDummyAddresses = () => {
  const addresses = [];
  for (let i = 0; i < 100; i++) {
    addresses.push({
      address: `0x${Math.random().toString(16).substring(2, 42).padStart(40, '0')}`,
      rank: i + 1,
      tradingVolume: Math.floor(Math.random() * 5000000) + 100000, // $100K to $5M
      buyCount: Math.floor(Math.random() * 500) + 10, // 10 to 500 buys
      sellCount: Math.floor(Math.random() * 400) + 5, // 5 to 400 sells
      lastActive: `${Math.floor(Math.random() * 30) + 1}d ago`
    });
  }
  return addresses;
};

const LandingPage = () => {
  const [currentPage, setCurrentPage] = useState(1);
  const [addresses, setAddresses] = useState<any[]>([]);
  const [isLoaded, setIsLoaded] = useState(false);
  const itemsPerPage = 10;

  useEffect(() => {
    // Generate data only on client side to avoid hydration mismatch
    setAddresses(generateDummyAddresses());
    setIsLoaded(true);
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

  if (!isLoaded) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center">
        <div className="text-white text-xl">Loading...</div>
      </div>
    );
  }

  const startItem = (currentPage - 1) * itemsPerPage + 1;
  const endItem = Math.min(currentPage * itemsPerPage, addresses.length);

  return (
    <div className="min-h-screen bg-gray-900">
      <Header />
      
      <main className="max-w-4xl mx-auto px-4 py-8">
        {/* Results counter and pagination at top */}
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
        </div>

        <div className="space-y-4">
          {currentAddresses.map((item, index) => (
            <Card
              key={item.address}
              address={item.address}
              rank={item.rank}
              tradingVolume={item.tradingVolume}
              buyCount={item.buyCount}
              sellCount={item.sellCount}
              lastActive={item.lastActive}
            />
          ))}
        </div>
      </main>
    </div>
  );
};

export default LandingPage;