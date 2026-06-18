self.addEventListener('push', (event) => {
  let payload = {
    title: 'Bourbask Veille',
    body: 'Nouveaux articles disponibles',
    url: '/veille',
  };
  if (event.data) {
    try { payload = event.data.json(); } catch {}
  }
  event.waitUntil(
    self.registration.showNotification(payload.title, {
      body: payload.body,
      icon: '/public/icons/android-chrome-192x192.png',
      badge: '/public/icons/favicon.ico',
      data: { url: payload.url },
      vibrate: [200, 100, 200],
    }),
  );
});

self.addEventListener('notificationclick', (event) => {
  event.notification.close();
  event.waitUntil(clients.openWindow(event.notification.data.url));
});
