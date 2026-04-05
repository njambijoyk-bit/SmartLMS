import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Bell, Check, CheckCheck, Trash2, Filter,
  AlertCircle, Info, CheckCircle2, AlertTriangle,
  BookOpen, FileText, Video, MessageSquare, Award,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Button } from '../../components/ui/Button';

type NotifType = 'info' | 'warning' | 'success' | 'danger';
type NotifCategory = 'assessment' | 'course' | 'live' | 'message' | 'achievement' | 'system';

interface Notification {
  id: string;
  title: string;
  message: string;
  type: NotifType;
  category: NotifCategory;
  time: string;
  read: boolean;
  actionLabel?: string;
}

const NOTIFICATIONS: Notification[] = [
  { id: '1', title: 'CAT 2 starts in 2 hours', message: 'CS301 CAT 2 — Binary Trees & Hash Tables opens at 2:00 PM today. Make sure you\'re prepared!', type: 'danger', category: 'assessment', time: '10 min ago', read: false, actionLabel: 'View CAT' },
  { id: '2', title: 'New announcement: Assignment 3 extended', message: 'Dr. Achieng has extended the CS302 Assignment 3 deadline to April 12, 11:59 PM.', type: 'info', category: 'course', time: '1 hour ago', read: false, actionLabel: 'View Assignment' },
  { id: '3', title: 'CAT 1 results published', message: 'Your MAT301 CAT 1 result is now available. You scored 88/100.', type: 'success', category: 'assessment', time: '3 hours ago', read: false, actionLabel: 'View Result' },
  { id: '4', title: 'Live class starting soon', message: 'Prof. Mwangi\'s SQL Joins session starts in 15 minutes. Join the Zoom room.', type: 'info', category: 'live', time: 'Yesterday', read: true, actionLabel: 'Join Now' },
  { id: '5', title: 'New message from Prof. Mwangi', message: 'Please review chapters 7 and 8 before the CAT 2. Focus on AVL tree rotations.', type: 'info', category: 'message', time: 'Yesterday', read: true, actionLabel: 'Reply' },
  { id: '6', title: 'Badge earned: Perfect Attendance', message: 'Congratulations! You have attended 10 consecutive classes without a miss.', type: 'success', category: 'achievement', time: '2 days ago', read: true, actionLabel: 'View Badge' },
  { id: '7', title: 'Assignment 2 graded', message: 'Your CS301 Algorithm Analysis assignment has been graded. Score: 82/100.', type: 'success', category: 'assessment', time: '3 days ago', read: true, actionLabel: 'View Feedback' },
  { id: '8', title: 'Upcoming exam: MAT301 End of Semester', message: 'The MAT301 End of Semester Exam is scheduled for May 2, 9:00 AM. Download your exam card.', type: 'warning', category: 'assessment', time: '4 days ago', read: true, actionLabel: 'Get Exam Card' },
  { id: '9', title: 'Course material updated', message: 'Prof. Kariuki has added new lecture notes for Week 8 of MAT301.', type: 'info', category: 'course', time: '5 days ago', read: true },
  { id: '10', title: 'System maintenance scheduled', message: 'SmartLMS will be briefly unavailable on Apr 7, 2:00–3:00 AM for scheduled maintenance.', type: 'warning', category: 'system', time: '6 days ago', read: true },
];

const TYPE_ICON: Record<NotifType, React.ReactNode> = {
  info: <Info size={16} className="text-info" />,
  warning: <AlertTriangle size={16} className="text-warning" />,
  success: <CheckCircle2 size={16} className="text-success" />,
  danger: <AlertCircle size={16} className="text-danger" />,
};

const CAT_ICON: Record<NotifCategory, React.ReactNode> = {
  assessment: <FileText size={14} />,
  course: <BookOpen size={14} />,
  live: <Video size={14} />,
  message: <MessageSquare size={14} />,
  achievement: <Award size={14} />,
  system: <Info size={14} />,
};

const TYPE_BG: Record<NotifType, string> = {
  info: 'bg-info-light',
  warning: 'bg-warning-light',
  success: 'bg-success-light',
  danger: 'bg-danger-light',
};

const CATEGORIES: { key: NotifCategory | 'all'; label: string }[] = [
  { key: 'all', label: 'All' },
  { key: 'assessment', label: 'Assessments' },
  { key: 'course', label: 'Courses' },
  { key: 'live', label: 'Live Classes' },
  { key: 'message', label: 'Messages' },
  { key: 'achievement', label: 'Achievements' },
  { key: 'system', label: 'System' },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function NotificationsPage() {
  const [notifications, setNotifications] = useState(NOTIFICATIONS);
  const [category, setCategory] = useState<NotifCategory | 'all'>('all');
  const [showUnread, setShowUnread] = useState(false);

  const unreadCount = notifications.filter(n => !n.read).length;

  const filtered = notifications.filter(n => {
    if (showUnread && n.read) return false;
    if (category !== 'all' && n.category !== category) return false;
    return true;
  });

  const markAllRead = () => setNotifications(prev => prev.map(n => ({ ...n, read: true })));
  const markRead = (id: string) => setNotifications(prev => prev.map(n => n.id === id ? { ...n, read: true } : n));
  const deleteNotif = (id: string) => setNotifications(prev => prev.filter(n => n.id !== id));

  return (
    <div className="space-y-5 max-w-3xl">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Notifications</h1>
              {unreadCount > 0 && (
                <span className="px-2 py-0.5 rounded-full bg-brand-500 text-white text-xs font-bold">{unreadCount}</span>
              )}
            </div>
            <p className="text-sm text-ink-tertiary mt-1">{unreadCount > 0 ? `${unreadCount} unread` : 'All caught up!'}</p>
          </div>
          <div className="flex gap-2">
            <Button variant="ghost" size="sm" onClick={() => setShowUnread(!showUnread)}>
              <Filter size={14} /> {showUnread ? 'Show all' : 'Unread only'}
            </Button>
            {unreadCount > 0 && (
              <Button variant="outline" size="sm" onClick={markAllRead}>
                <CheckCheck size={14} /> Mark all read
              </Button>
            )}
          </div>
        </div>
      </motion.div>

      {/* Category filter */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
        <div className="flex gap-2 overflow-x-auto pb-1">
          {CATEGORIES.map(c => (
            <button key={c.key} onClick={() => setCategory(c.key)}
              className={`px-3 py-1.5 text-xs font-medium rounded-full whitespace-nowrap transition-all cursor-pointer ${category === c.key ? 'bg-brand-500 text-white' : 'bg-surface-raised border border-sand-300 text-ink-secondary hover:border-brand-300'}`}>
              {c.label}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Notifications list */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }}>
        {filtered.length === 0 ? (
          <Card className="text-center py-16">
            <Bell size={36} className="mx-auto text-ink-placeholder mb-3" />
            <p className="text-ink-tertiary font-medium">No notifications</p>
            <p className="text-sm text-ink-placeholder mt-1">You're all caught up</p>
          </Card>
        ) : (
          <div className="space-y-2">
            {filtered.map((notif, i) => (
              <motion.div
                key={notif.id}
                initial={{ opacity: 0, x: -6 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.04 }}
                layout
              >
                <div className={`relative rounded-xl border transition-all ${notif.read ? 'bg-surface-raised border-sand-200' : `${TYPE_BG[notif.type]} border-${notif.type === 'info' ? 'info' : notif.type === 'success' ? 'success' : notif.type === 'warning' ? 'warning' : 'danger'}/20`}`}>
                  {!notif.read && (
                    <div className="absolute left-4 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-brand-500" />
                  )}
                  <div className={`p-4 ${!notif.read ? 'pl-9' : 'pl-4'}`}>
                    <div className="flex items-start gap-3">
                      <div className={`w-9 h-9 rounded-xl ${TYPE_BG[notif.type]} flex items-center justify-center shrink-0 mt-0.5`}>
                        {TYPE_ICON[notif.type]}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-start justify-between gap-2">
                          <h4 className={`text-sm font-semibold font-[family-name:var(--font-display)] ${notif.read ? 'text-ink-secondary' : 'text-ink'}`}>{notif.title}</h4>
                          <span className="text-[10px] text-ink-placeholder whitespace-nowrap mt-0.5">{notif.time}</span>
                        </div>
                        <p className="text-xs text-ink-tertiary mt-0.5 leading-relaxed">{notif.message}</p>
                        <div className="flex items-center gap-3 mt-2.5">
                          <span className={`flex items-center gap-1 text-[10px] px-2 py-0.5 rounded-full bg-sand-100 text-ink-tertiary font-medium capitalize`}>
                            {CAT_ICON[notif.category]} {notif.category}
                          </span>
                          {notif.actionLabel && (
                            <button className="text-xs font-semibold text-brand-500 hover:text-brand-600 cursor-pointer">
                              {notif.actionLabel} →
                            </button>
                          )}
                        </div>
                      </div>
                      <div className="flex gap-1 shrink-0">
                        {!notif.read && (
                          <button onClick={() => markRead(notif.id)} className="p-1.5 rounded-lg hover:bg-sand-200 text-ink-tertiary cursor-pointer" title="Mark as read">
                            <Check size={14} />
                          </button>
                        )}
                        <button onClick={() => deleteNotif(notif.id)} className="p-1.5 rounded-lg hover:bg-danger-light text-ink-tertiary hover:text-danger cursor-pointer" title="Delete">
                          <Trash2 size={14} />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </motion.div>
            ))}
          </div>
        )}
      </motion.div>
    </div>
  );
}
