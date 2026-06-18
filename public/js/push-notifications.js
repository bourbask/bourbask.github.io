(async function () {
  if (!('serviceWorker' in navigator) || !('PushManager' in window)) return;

  const VAPID_PUBLIC_KEY =
    'BKMPXIKrefn3zVnS6DF-ImYQDsFjuD4c49jfaKVEs8bYkMFTNT_mlgvFRv9R-UUUS3RsK-R-dbWmRYCwr4ql9gA';
  const WORKER_URL = 'https://bourbask-contact.bourbask.workers.dev';

  function urlBase64ToUint8Array(base64String) {
    var padding = '='.repeat((4 - (base64String.length % 4)) % 4);
    var base64 = (base64String + padding).replace(/-/g, '+').replace(/_/g, '/');
    var rawData = window.atob(base64);
    var output = new Uint8Array(rawData.length);
    for (var i = 0; i < rawData.length; i++) {
      output[i] = rawData.charCodeAt(i);
    }
    return output;
  }

  try {
    var registration = await navigator.serviceWorker.register('/sw.js');

    var permission = await Notification.requestPermission();
    if (permission !== 'granted') return;

    var subscription = await registration.pushManager.subscribe({
      userVisibleOnly: true,
      applicationServerKey: urlBase64ToUint8Array(VAPID_PUBLIC_KEY),
    });

    await fetch(WORKER_URL + '/sub/subscribe', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(subscription.toJSON()),
    });
  } catch (e) {
    console.warn('Push notification setup failed:', e);
  }
})();
