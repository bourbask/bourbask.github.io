import { createRequire } from 'module';
import { readFileSync } from 'fs';

const require = createRequire(import.meta.url);
const webpush = require('web-push');

const WORKER_URL = process.env.WORKER_URL || 'https://bourbask-contact.bourbask.workers.dev';
const NOTIFY_SECRET = process.env.NOTIFY_SECRET;
const VAPID_SUBJECT = process.env.VAPID_SUBJECT || 'mailto:bourbasquet.k@etik.com';
const VAPID_PUBLIC_KEY = process.env.VAPID_PUBLIC_KEY;
const VAPID_PRIVATE_KEY = process.env.VAPID_PRIVATE_KEY;

if (!NOTIFY_SECRET || !VAPID_PUBLIC_KEY || !VAPID_PRIVATE_KEY) {
  console.error('Missing required env: NOTIFY_SECRET, VAPID_PUBLIC_KEY, VAPID_PRIVATE_KEY');
  process.exit(1);
}

webpush.setVapidDetails(VAPID_SUBJECT, VAPID_PUBLIC_KEY, VAPID_PRIVATE_KEY);

// Count recent selected articles
let articleCount = 0;
try {
  const raw = readFileSync('public/news.json', 'utf8');
  const news = JSON.parse(raw);
  if (news.articles) {
    articleCount = news.articles.filter((a) => {
      const rawDate = a.date_created || a.pub_date || '';
      const ageDays = (Date.now() - new Date(rawDate).getTime()) / 86400000;
      return a.status === 'selected' && ageDays < 3;
    }).length;
  }
} catch (e) {
  console.warn('Could not parse news.json, sending generic notification');
}

if (articleCount === 0) {
  console.log('No recent selected articles found, skipping notification');
  process.exit(0);
}

// Fetch subscriptions from worker
const resp = await fetch(`${WORKER_URL}/sub/subs`, {
  headers: { 'X-Notify-Secret': NOTIFY_SECRET },
});
if (!resp.ok) {
  console.error(`Failed to fetch subscriptions (${resp.status})`);
  process.exit(1);
}
const subs = await resp.json();

if (!Array.isArray(subs) || subs.length === 0) {
  console.log('No push subscribers to notify');
  process.exit(0);
}

const payload = JSON.stringify({
  title: 'Nouveaux articles sur la veille',
  body: articleCount > 0
    ? `${articleCount} nouvel${articleCount > 1 ? 's' : ''} article${articleCount > 1 ? 's' : ''} — consultez la veille`
    : 'La veille technologique a été mise à jour',
  url: '/veille',
});

let sent = 0;
let invalid = [];

for (const sub of subs) {
  try {
    await webpush.sendNotification(sub, payload);
    sent++;
  } catch (err) {
    if (err.statusCode === 410 || err.statusCode === 404) {
      invalid.push(sub.endpoint);
    } else {
      console.error(`Failed to send to ${sub.endpoint}: ${err.message}`);
    }
  }
}

// Clean up invalid subscriptions
if (invalid.length > 0) {
  try {
    await fetch(`${WORKER_URL}/sub/unsubscribe`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Notify-Secret': NOTIFY_SECRET,
      },
      body: JSON.stringify({ endpoints: invalid }),
    });
  } catch (e) {
    console.warn('Failed to clean up invalid subscriptions:', e.message);
  }
}

console.log(`Notified ${sent}/${subs.length} subscriber${subs.length > 1 ? 's' : ''}, removed ${invalid.length} invalid`);
