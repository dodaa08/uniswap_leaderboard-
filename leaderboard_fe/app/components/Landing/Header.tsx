import React from 'react';
import Link from 'next/link';
import { FaGithub } from "react-icons/fa";
import { FaShareSquare } from "react-icons/fa";

interface HeaderProps {
  onSync?: () => void;
  isSyncing?: boolean;
}

const Header = ({ onSync, isSyncing = false }: HeaderProps) => {
  return (
    <header className="bg-gray-900 text-white py-8 px-4 relative">
      <div className="max-w-6xl mx-auto text-center">
        <h1 className="text-4xl md:text-5xl font-bold text-white bg-clip-text text-transparent">
          Leaderboard
        </h1>
        <p className="text-gray-300 mt-2 text-lg">
          Uniswap V3 Leaderboard for Zora Token
        </p>
        
       <Link target='_blank' href="https://dexscreener.com/base/0xedc625b74537ee3a10874f53d170e9c17a906b9c">
        <div className='flex justify-center items-center gap-2 border-2 border-gray-800 w-max py-2 px-5 rounded-xl mt-4 cursor-pointer hover:bg-gray-800 transition-all duration-300 mx-auto'>
          <span className="text-white text-sm font-medium">Zora Token</span>
          <FaShareSquare className='text-gray-400' size={14} />
        </div>
        </Link>
      </div>
      
      {/* Top right buttons */}
      <div className="absolute top-6 right-6 flex items-center space-x-3">
        {/* Sync button */}
        {onSync && (
          <button
            onClick={onSync}
            disabled={isSyncing}
            className="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-3 py-2 rounded-lg transition-colors flex items-center space-x-2 text-sm"
            title="Sync latest data from Uniswap"
          >
            {isSyncing ? (
              <>
                <div className="animate-spin rounded-full h-3 w-3 border-b-2 border-white"></div>
                <span>Syncing</span>
              </>
            ) : (
              <>
                <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
                <span>Sync</span>
              </>
            )}
          </button>
        )}
        
        {/* GitHub icon */}
        <Link 
          className='hover:bg-gray-50 transition-transform duration-400 bg-gray-100 rounded-full p-2' 
          target='_blank' 
          href="https://github.com/dodaa08/uniswap_leaderboard-"
          title="View on GitHub"
        >
          <FaGithub className='text-gray-900 hover:text-black transition-colors' size={20} />
        </Link>
      </div>
    
    </header>
  );
};

export default Header;
