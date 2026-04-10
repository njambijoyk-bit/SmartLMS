import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Search, Send, Paperclip, MoreHorizontal, Phone,
  Video, ChevronLeft, Edit, Check, CheckCheck,
} from 'lucide-react';

interface Contact {
  id: string;
  name: string;
  role: string;
  lastMessage: string;
  time: string;
  unread: number;
  online: boolean;
  initials: string;
  color: string;
}

interface Message {
  id: string;
  from: 'me' | 'them';
  text: string;
  time: string;
  read?: boolean;
}

const CONTACTS: Contact[] = [
  { id: '1', name: 'Prof. James Mwangi', role: 'Instructor · CS301', lastMessage: 'The CAT 2 will cover Binary Trees and Hash Tables...', time: '10:24 AM', unread: 2, online: true, initials: 'JM', color: 'bg-brand-500' },
  { id: '2', name: 'Dr. Achieng Odhiambo', role: 'Instructor · CS302', lastMessage: 'Assignment 3 deadline extended to Apr 12.', time: 'Yesterday', unread: 0, online: false, initials: 'AO', color: 'bg-accent-400' },
  { id: '3', name: 'Faith Kamau', role: 'Learner · CS301', lastMessage: 'Thank you for the feedback!', time: 'Yesterday', unread: 0, online: true, initials: 'FK', color: 'bg-gold-500' },
  { id: '4', name: 'Admin Office', role: 'Administration', lastMessage: 'Your exam card is ready for download.', time: 'Apr 2', unread: 1, online: true, initials: 'AO', color: 'bg-brand-700' },
  { id: '5', name: 'Daniel Mutua', role: 'Learner · MAT301', lastMessage: "I'm struggling with graph colouring, can we meet?", time: 'Apr 1', unread: 0, online: false, initials: 'DM', color: 'bg-accent-300' },
  { id: '6', name: 'CS301 Study Group', role: '12 members', lastMessage: 'Brian: Anyone done the practice problems?', time: 'Mar 31', unread: 5, online: false, initials: 'SG', color: 'bg-brand-300' },
];

const MESSAGES: Record<string, Message[]> = {
  '1': [
    { id: '1', from: 'them', text: 'Good morning! Just a reminder that CAT 2 is scheduled for tomorrow at 2:00 PM.', time: '9:05 AM' },
    { id: '2', from: 'them', text: 'The CAT will cover Binary Trees and Hash Tables. Please review chapters 7 and 8.', time: '9:06 AM' },
    { id: '3', from: 'me', text: 'Thank you, Professor. Can you confirm if we need to know AVL tree rotations?', time: '9:32 AM', read: true },
    { id: '4', from: 'them', text: 'Yes, single and double rotations are included. Focus on understanding the balance factor concept.', time: '9:45 AM' },
    { id: '5', from: 'me', text: 'Understood. Will the questions be multiple choice or structured?', time: '10:01 AM', read: true },
    { id: '6', from: 'them', text: 'The CAT 2 will cover Binary Trees and Hash Tables. Mix of MCQ (10 marks) and short answer (20 marks). Good luck!', time: '10:24 AM' },
  ],
};

export function MessagesPage() {
  const [selected, setSelected] = useState<string | null>('1');
  const [message, setMessage] = useState('');
  const [mobileView, setMobileView] = useState<'list' | 'chat'>('list');
  const [searchQuery, setSearchQuery] = useState('');

  const contact = CONTACTS.find(c => c.id === selected);
  const messages = selected ? (MESSAGES[selected] || []) : [];

  const filteredContacts = CONTACTS.filter(c =>
    c.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const handleSelect = (id: string) => {
    setSelected(id);
    setMobileView('chat');
  };

  return (
    <div className="flex h-[calc(100vh-5rem)] -m-6 overflow-hidden rounded-xl">
      {/* Sidebar — contact list */}
      <div className={`w-full md:w-80 border-r border-sand-200 bg-surface-raised flex flex-col shrink-0 ${mobileView === 'chat' ? 'hidden md:flex' : 'flex'}`}>
        {/* Header */}
        <div className="p-4 border-b border-sand-200">
          <div className="flex items-center justify-between mb-3">
            <h2 className="font-bold font-[family-name:var(--font-display)] text-ink">Messages</h2>
            <button className="p-1.5 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer">
              <Edit size={17} />
            </button>
          </div>
          <div className="relative">
            <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input
              type="text"
              placeholder="Search messages..."
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="w-full bg-sand-100 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:bg-surface-raised focus:ring-2 focus:ring-brand-300 transition-all"
            />
          </div>
        </div>

        {/* Contact list */}
        <div className="flex-1 overflow-y-auto">
          {filteredContacts.map(contact => (
            <motion.button
              key={contact.id}
              whileTap={{ scale: 0.98 }}
              onClick={() => handleSelect(contact.id)}
              className={`w-full flex items-center gap-3 px-4 py-3.5 hover:bg-sand-50 transition-colors text-left cursor-pointer ${selected === contact.id ? 'bg-brand-50 border-r-2 border-brand-500' : ''}`}
            >
              {/* Avatar */}
              <div className="relative shrink-0">
                <div className={`w-11 h-11 rounded-full ${contact.color} flex items-center justify-center text-white text-sm font-bold font-[family-name:var(--font-display)]`}>
                  {contact.initials}
                </div>
                {contact.online && (
                  <span className="absolute bottom-0 right-0 w-3 h-3 rounded-full bg-success border-2 border-white" />
                )}
              </div>

              {/* Text */}
              <div className="flex-1 min-w-0">
                <div className="flex items-center justify-between">
                  <span className={`text-sm font-semibold truncate ${selected === contact.id ? 'text-brand-600' : 'text-ink'}`}>{contact.name}</span>
                  <span className="text-[10px] text-ink-tertiary whitespace-nowrap ml-2">{contact.time}</span>
                </div>
                <div className="text-xs text-ink-tertiary truncate">{contact.role}</div>
                <div className="flex items-center justify-between mt-0.5">
                  <span className="text-xs text-ink-placeholder truncate max-w-[160px]">{contact.lastMessage}</span>
                  {contact.unread > 0 && (
                    <span className="ml-2 min-w-[18px] h-[18px] rounded-full bg-brand-500 text-white text-[10px] font-bold flex items-center justify-center px-1 shrink-0">
                      {contact.unread}
                    </span>
                  )}
                </div>
              </div>
            </motion.button>
          ))}
        </div>
      </div>

      {/* Chat area */}
      {selected && contact ? (
        <div className={`flex-1 flex flex-col bg-surface min-w-0 ${mobileView === 'list' ? 'hidden md:flex' : 'flex'}`}>
          {/* Chat header */}
          <div className="h-16 border-b border-sand-200 bg-surface-raised px-5 flex items-center gap-3 shrink-0">
            <button
              onClick={() => setMobileView('list')}
              className="md:hidden p-1 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer mr-1"
            >
              <ChevronLeft size={20} />
            </button>
            <div className={`w-9 h-9 rounded-full ${contact.color} flex items-center justify-center text-white text-sm font-bold font-[family-name:var(--font-display)] shrink-0`}>
              {contact.initials}
            </div>
            <div className="flex-1 min-w-0">
              <div className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{contact.name}</div>
              <div className="text-xs text-ink-tertiary flex items-center gap-1.5">
                {contact.online && <span className="w-1.5 h-1.5 rounded-full bg-success" />}
                {contact.online ? 'Online now' : contact.role}
              </div>
            </div>
            <div className="flex items-center gap-1">
              <button className="p-2 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer">
                <Phone size={17} />
              </button>
              <button className="p-2 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer">
                <Video size={17} />
              </button>
              <button className="p-2 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer">
                <MoreHorizontal size={17} />
              </button>
            </div>
          </div>

          {/* Messages */}
          <div className="flex-1 overflow-y-auto px-5 py-5 space-y-3">
            {messages.length === 0 && (
              <div className="flex flex-col items-center justify-center h-full text-ink-tertiary">
                <div className={`w-16 h-16 rounded-full ${contact.color} flex items-center justify-center text-white text-xl font-bold font-[family-name:var(--font-display)] mb-4`}>
                  {contact.initials}
                </div>
                <p className="text-sm font-medium">{contact.name}</p>
                <p className="text-xs mt-1">No messages yet. Start the conversation!</p>
              </div>
            )}
            {messages.map((msg, i) => {
              const isMe = msg.from === 'me';
              return (
                <motion.div
                  key={msg.id}
                  initial={{ opacity: 0, y: 8 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: i * 0.04 }}
                  className={`flex ${isMe ? 'justify-end' : 'justify-start'}`}
                >
                  <div className={`max-w-[72%] ${isMe ? 'items-end' : 'items-start'} flex flex-col gap-1`}>
                    <div className={`px-4 py-2.5 rounded-2xl text-sm leading-relaxed ${
                      isMe
                        ? 'bg-brand-500 text-white rounded-br-sm'
                        : 'bg-surface-raised border border-sand-200 text-ink rounded-bl-sm'
                    }`}>
                      {msg.text}
                    </div>
                    <div className={`flex items-center gap-1 text-[10px] text-ink-tertiary ${isMe ? 'flex-row-reverse' : ''}`}>
                      <span>{msg.time}</span>
                      {isMe && (
                        msg.read ? <CheckCheck size={11} className="text-brand-300" /> : <Check size={11} />
                      )}
                    </div>
                  </div>
                </motion.div>
              );
            })}
          </div>

          {/* Input */}
          <div className="border-t border-sand-200 bg-surface-raised p-4">
            <div className="flex items-center gap-3">
              <button className="p-2 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer shrink-0">
                <Paperclip size={18} />
              </button>
              <div className="flex-1 bg-sand-100 rounded-xl px-4 py-2.5 flex items-center gap-2">
                <input
                  type="text"
                  placeholder="Type a message..."
                  value={message}
                  onChange={e => setMessage(e.target.value)}
                  onKeyDown={e => e.key === 'Enter' && setMessage('')}
                  className="flex-1 bg-transparent text-sm focus:outline-none text-ink placeholder:text-ink-placeholder"
                />
              </div>
              <button
                className={`p-2.5 rounded-xl transition-all cursor-pointer shrink-0 ${
                  message.trim() ? 'bg-brand-500 text-white hover:bg-brand-600' : 'bg-sand-200 text-ink-placeholder cursor-not-allowed'
                }`}
                onClick={() => setMessage('')}
              >
                <Send size={17} />
              </button>
            </div>
          </div>
        </div>
      ) : (
        <div className="flex-1 flex flex-col items-center justify-center text-ink-tertiary">
          <div className="w-16 h-16 rounded-full bg-sand-200 flex items-center justify-center mb-4">
            <Send size={24} className="text-ink-placeholder" />
          </div>
          <p className="text-sm font-medium">Select a conversation</p>
          <p className="text-xs mt-1">Choose from your messages on the left</p>
        </div>
      )}
    </div>
  );
}
