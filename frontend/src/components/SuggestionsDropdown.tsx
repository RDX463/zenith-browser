import { Bookmark, History } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { cn } from '../lib/utils';
import { ipc } from '../ipc';
import type { Suggestion } from '../ipc';

interface SuggestionsDropdownProps {
  suggestions: Suggestion[];
  selectedIndex: number;
  activeId: number | null;
  onSuggestionsClose: () => void;
}

export default function SuggestionsDropdown({
  suggestions,
  selectedIndex,
  activeId,
  onSuggestionsClose,
}: SuggestionsDropdownProps) {
  if (suggestions.length === 0) return null;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, y: -10 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -10 }}
        className="absolute top-[82px] left-1/2 -translate-x-1/2 w-[600px] glass rounded-2xl overflow-hidden shadow-2xl z-50 mt-2"
      >
        <div className="p-2 flex flex-col gap-1">
          {suggestions.map((s, i) => (
            <div 
              key={`${s.url}-${i}`}
              className={cn(
                "flex items-center gap-3 px-4 py-3 rounded-xl cursor-pointer transition-colors",
                selectedIndex === i ? "bg-white/10" : "hover:bg-white/5"
              )}
              onClick={() => {
                ipc.send({ type: 'navigate', tabId: activeId!, url: s.url || s.title });
                onSuggestionsClose();
              }}
            >
              <SuggestionIcon type={s.suggestionType} />
              <div className="flex flex-col">
                <span className="text-sm font-medium">{s.title}</span>
                {s.url && <span className="text-xs text-zenith-text-muted truncate">{s.url}</span>}
              </div>
            </div>
          ))}
        </div>
      </motion.div>
    </AnimatePresence>
  );
}

function SuggestionIcon({ type }: { type: string }) {
  if (type === 'bookmark') return <Bookmark size={16} className="text-yellow-400" fill="currentColor" />;
  if (type === 'tab') return <div className="w-2 h-2 rounded-full bg-zenith-primary shadow-[0_0_8px_rgba(59,130,246,0.5)]" />;
  return <History size={16} className="text-zenith-text-muted" />;
}
