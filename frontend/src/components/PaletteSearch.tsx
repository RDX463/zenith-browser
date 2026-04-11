import { Search } from 'lucide-react';
import { motion } from 'framer-motion';
import { ipc } from '../ipc';

interface PaletteSearchProps {
  searchQuery: string;
  onSearchChange: (query: string) => void;
}

export default function PaletteSearch({ searchQuery, onSearchChange }: PaletteSearchProps) {
  return (
    <div className="absolute inset-0 bg-black/40 backdrop-blur-sm z-[100] flex items-start justify-center pt-[15vh]">
      <motion.div 
        initial={{ scale: 0.95, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        className="w-[600px] glass rounded-3xl overflow-hidden shadow-2xl border-white/20"
      >
        <div className="flex items-center gap-4 px-6 py-5 border-b border-white/10">
          <Search className="text-zenith-primary" size={24} />
          <input 
            autoFocus
            className="flex-1 bg-transparent text-xl focus:outline-none placeholder:text-white/20"
            placeholder="Search, type URL, or command..."
            onChange={(e) => {
              onSearchChange(e.target.value);
              ipc.send({ type: 'get_suggestions', query: e.target.value });
            }}
            value={searchQuery}
          />
        </div>
      </motion.div>
    </div>
  );
}
