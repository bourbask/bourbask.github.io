// FAB radial drag interaction
// Long-press (or mousedown hold) opens drag mode; pointer moves FAB toward arc
// items; magnetic snap + glassy center label on confirm; release triggers action.
// Works on both touch (mobile) and mouse (desktop).
(function () {
  'use strict';

  var LONG_PRESS_MS   = 220;   // hold to enter drag mode
  var MAGNETIC_DIST   = 90;    // px — start pulling toward item
  var TRIGGER_DIST    = 48;    // px — snap & confirm action
  var MAX_DRAG_RADIUS = 185;   // px — clamp FAB travel

  var fab, navItemsEl, overlay, glassyLabel;
  var isDragging      = false;
  var tapOpen         = false;
  var pressTimer      = null;
  var fabOriginX      = 0;
  var fabOriginY      = 0;
  var confirmedEl     = null;
  var pointerActive   = false;  // set only on FAB start — guards stray move/end

  // ── Coordinate helper ──────────────────────────────────────────────────────
  function getXY(e) {
    if (e.touches && e.touches[0]) return { x: e.touches[0].clientX, y: e.touches[0].clientY };
    return { x: e.clientX, y: e.clientY };
  }

  // ── Init ──────────────────────────────────────────────────────────────────
  function init() {
    fab        = document.getElementById('mobileFab');
    navItemsEl = document.getElementById('mobileNavItems');
    overlay    = document.getElementById('mobileNavOverlay');
    if (!fab) return;

    if (!document.getElementById('fabGlassyLabel')) {
      glassyLabel = document.createElement('div');
      glassyLabel.id        = 'fabGlassyLabel';
      glassyLabel.className = 'fab-glassy-label';
      document.body.appendChild(glassyLabel);
    } else {
      glassyLabel = document.getElementById('fabGlassyLabel');
    }

    // Touch (mobile)
    fab.addEventListener('touchstart',  onStart, { passive: false });
    document.addEventListener('touchmove',   onMove,  { passive: false });
    document.addEventListener('touchend',    onEnd);
    document.addEventListener('touchcancel', onEnd);

    // Mouse (desktop)
    fab.addEventListener('mousedown', onStart);
    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup',   onEnd);

    overlay && overlay.addEventListener('click', function () {
      if (tapOpen && !isDragging) closeTapMenu();
    });
  }

  // ── Tap mode ──────────────────────────────────────────────────────────────
  function openTapMenu() {
    tapOpen = true;
    fab.classList.add('active');
    navItemsEl && navItemsEl.classList.add('active');
    overlay    && overlay.classList.add('active');
    document.body.classList.add('mobile-nav-open');
  }

  function closeTapMenu() {
    tapOpen = false;
    fab.classList.remove('active');
    navItemsEl && navItemsEl.classList.remove('active');
    overlay    && overlay.classList.remove('active');
    document.body.classList.remove('mobile-nav-open');
  }

  // ── Drag mode ─────────────────────────────────────────────────────────────
  function openDragMenu(cx, cy) {
    isDragging = true;
    fabOriginX = cx;
    fabOriginY = cy;

    fab.style.transition = 'none';
    fab.classList.add('drag-open');
    navItemsEl && navItemsEl.classList.add('active', 'drag-hint');
    overlay    && overlay.classList.add('active');
    document.body.classList.add('mobile-nav-open');
  }

  function closeDragMenu(trigger) {
    isDragging  = false;
    confirmedEl = null;

    fab.style.transition = 'transform 0.35s cubic-bezier(0.34, 1.56, 0.64, 1)';
    fab.style.transform  = '';

    fab.classList.remove('drag-open', 'near-target', 'ready-trigger', 'touched');
    navItemsEl && navItemsEl.classList.remove('active', 'drag-hint');
    overlay    && overlay.classList.remove('active');
    document.body.classList.remove('mobile-nav-open');

    document.querySelectorAll('.mobile-nav-item')
      .forEach(function (el) { el.classList.remove('near-target', 'ready-trigger', 'activated'); });

    glassyLabel && glassyLabel.classList.remove('active', 'confirmed');

    setTimeout(function () { fab.style.transition = ''; }, 380);

    if (trigger) {
      trigger.classList.add('activated');
      setTimeout(function () {
        trigger.click();
        trigger.classList.remove('activated');
      }, 60);
    }
  }

  // ── Unified pointer handlers ───────────────────────────────────────────────
  function onStart(e) {
    if (e.type === 'touchstart') e.preventDefault();
    // Ignore right-click on desktop
    if (e.button && e.button !== 0) return;

    pointerActive = true;
    fab.classList.add('touched');

    var rect = fab.getBoundingClientRect();
    var cx   = rect.left + rect.width  / 2;
    var cy   = rect.top  + rect.height / 2;

    pressTimer = setTimeout(function () { openDragMenu(cx, cy); }, LONG_PRESS_MS);
  }

  function onMove(e) {
    if (!pointerActive) return;

    // Prevent browser from capturing the touch as a scroll gesture
    // while we're holding a press on the FAB (before or during drag mode).
    if (e.type === 'touchmove') e.preventDefault();

    var pt = getXY(e);

    if (!isDragging) {
      // Slide while holding → enter drag mode immediately, no need to wait
      if (pressTimer) {
        var rect = fab.getBoundingClientRect();
        var cx   = rect.left + rect.width  / 2;
        var cy   = rect.top  + rect.height / 2;
        if (Math.hypot(pt.x - cx, pt.y - cy) > 12) {
          clearTimeout(pressTimer);
          pressTimer = null;
          openDragMenu(cx, cy);
        }
      }
      return;
    }

    var rawDx   = pt.x - fabOriginX;
    var rawDy   = pt.y - fabOriginY;
    var rawDist = Math.hypot(rawDx, rawDy);
    var clamp   = rawDist > MAX_DRAG_RADIUS ? MAX_DRAG_RADIUS / rawDist : 1;
    var movX    = rawDx * clamp;
    var movY    = rawDy * clamp;

    var fabCx = fabOriginX + movX;
    var fabCy = fabOriginY + movY;

    var closest  = null;
    var closestD = Infinity;
    document.querySelectorAll('.mobile-nav-item').forEach(function (el) {
      var r  = el.getBoundingClientRect();
      var ix = r.left + r.width  / 2;
      var iy = r.top  + r.height / 2;
      var d  = Math.hypot(fabCx - ix, fabCy - iy);
      if (d < closestD) { closestD = d; closest = { el: el, ix: ix, iy: iy }; }
    });

    var finalX = movX, finalY = movY;

    if (closest && closestD < TRIGGER_DIST) {
      finalX = closest.ix - fabOriginX;
      finalY = closest.iy - fabOriginY;
      fab.style.transition = 'transform 0.12s cubic-bezier(0.34, 1.56, 0.64, 1)';
      setItemState('ready', closest.el);
      showGlassy(closest.el.dataset.label, true);
      confirmedEl = closest.el;

    } else if (closest && closestD < MAGNETIC_DIST) {
      fab.style.transition = 'none';
      var t    = 1 - closestD / MAGNETIC_DIST;
      var pull = t * t * 0.55;
      var tx   = closest.ix - fabOriginX;
      var ty   = closest.iy - fabOriginY;
      finalX = movX + (tx - movX) * pull;
      finalY = movY + (ty - movY) * pull;
      setItemState('near', closest.el);
      showGlassy(closest.el.dataset.label, false);
      confirmedEl = null;

    } else {
      fab.style.transition = 'none';
      setItemState('none', null);
      glassyLabel && glassyLabel.classList.remove('active', 'confirmed');
      confirmedEl = null;
    }

    fab.style.transform = 'translate(' + finalX + 'px,' + finalY + 'px) scale(1.22)';
  }

  function onEnd() {
    clearTimeout(pressTimer);
    pressTimer = null;

    if (!pointerActive) return;
    pointerActive = false;

    fab.classList.remove('touched');

    if (!isDragging) {
      if (tapOpen) closeTapMenu(); else openTapMenu();
      return;
    }

    closeDragMenu(confirmedEl);
  }

  // ── Helpers ────────────────────────────────────────────────────────────────
  function setItemState(state, activeEl) {
    document.querySelectorAll('.mobile-nav-item')
      .forEach(function (el) { el.classList.remove('near-target', 'ready-trigger'); });
    fab.classList.remove('near-target', 'ready-trigger');

    if (state === 'ready' && activeEl) {
      fab.classList.add('near-target', 'ready-trigger');
      activeEl.classList.add('near-target', 'ready-trigger');
    } else if (state === 'near' && activeEl) {
      fab.classList.add('near-target');
      activeEl.classList.add('near-target');
    }
  }

  function showGlassy(label, confirmed) {
    if (!glassyLabel || !label) return;
    glassyLabel.textContent = label;
    glassyLabel.classList.add('active');
    glassyLabel.classList.toggle('confirmed', confirmed);
  }

  // ── Boot (wait for Leptos WASM render) ────────────────────────────────────
  function tryInit() {
    if (document.getElementById('mobileFab')) {
      init();
    } else {
      var obs = new MutationObserver(function () {
        if (document.getElementById('mobileFab')) {
          obs.disconnect();
          init();
        }
      });
      obs.observe(document.body, { childList: true, subtree: true });
    }
  }

  document.readyState === 'loading'
    ? document.addEventListener('DOMContentLoaded', tryInit)
    : tryInit();
})();
