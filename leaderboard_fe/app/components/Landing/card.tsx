import React, { useState } from 'react';
import { FaCopy, FaCheck } from 'react-icons/fa';

interface CardProps {
  address: string;
  rank: number;
  tradingVolume: number;
  buyCount: number;
  sellCount: number;
  lastActive: string;
}

const Card = ({ address, rank, tradingVolume, buyCount, sellCount, lastActive }: CardProps) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(address);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const truncateAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  const formatVolume = (volume: number) => {
    if (volume >= 1000000) {
      return `$${(volume / 1000000).toFixed(2)}M`;
    } else if (volume >= 1000) {
      return `$${(volume / 1000).toFixed(1)}K`;
    }
    return `$${volume.toLocaleString()}`;
  };

  return (
    <div className="bg-gray-800/50 border border-gray-700/50 rounded-xl p-6 hover:border-blue-500/50 transition-all duration-300 hover:bg-gray-800/70">
      <div className="flex items-center justify-between">
        {/* Left section with rank and address */}
        <div className="flex items-center space-x-4">
          <div className="text-gray-400 font-mono text-sm min-w-[3rem]">
            #{rank}
          </div>
          
          <div className="flex flex-col">
            <div className="flex items-center space-x-2 mb-2">
              <p className="text-white font-mono text-sm">
                {truncateAddress(address)}
              </p>
              <button
                onClick={handleCopy}
                className="text-gray-400 hover:text-white transition-colors p-1"
                title="Copy full address"
              >
                {copied ? <FaCheck size={14} className="text-green-400" /> : <FaCopy size={14} />}
              </button>
            </div>
            <p className="text-gray-400 text-xs">Last active: {lastActive}</p>
          </div>
        </div>

        {/* Right section with trading data */}
        <div className="flex items-center space-x-8">
          <div className="text-center">
            <p className="text-gray-400 text-xs uppercase tracking-wide">Trading Volume</p>
            <p className="text-green-400 font-bold text-lg">{formatVolume(tradingVolume)}</p>
          </div>
          
          <div className="text-center">
            <p className="text-gray-400 text-xs uppercase tracking-wide">Buys</p>
            <p className="text-blue-400 font-semibold text-lg">{buyCount.toLocaleString()}</p>
          </div>
          
          <div className="text-center">
            <p className="text-gray-400 text-xs uppercase tracking-wide">Sells</p>
            <p className="text-red-400 font-semibold text-lg">{sellCount.toLocaleString()}</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Card;
