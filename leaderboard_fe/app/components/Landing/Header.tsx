import React from 'react';
import Link from 'next/link';
import { FaGithub } from "react-icons/fa";
import { FaShareSquare } from "react-icons/fa";

const Header = () => {
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
      
      {/* GitHub icon positioned with some margin from right and white background */}
      <Link 
        className='absolute top-6 right-12 hover:bg-gray-50 transition-transform duration-400 bg-gray-100 rounded-full p-2 mt-3' 
        target='_blank' 
        href="https://github.com/dodaa08/uniswap_leaderboard-"
      >
        <FaGithub className='text-gray-900 hover:text-black transition-colors' size={24} />
      </Link>
    
    </header>
  );
};

export default Header;
