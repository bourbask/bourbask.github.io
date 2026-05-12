// FAB radial drag interaction
// Long-press opens drag mode; finger moves FAB toward arc items;
// magnetic snap + glassy center label on confirm; release triggers action.
(function () {
  'use strict';

  var LONG_PRESS_MS   = 220;   // hold to enter drag mode
  var MAGNETIC_DIST   = 90;    // px — start pulling toward item
  var TRIGGER_DIST    = 48;    // px — snap & confirm action
  var MAX_DRAG_RADIUS = 185;   // px — clamp FAB travel

  var fab, navItemsEl, overlay, glassyLabel;
  var isDragging  = false;
  var tapOpen     = false;
  var pressTimer  = null;
  var fabOriginX  = 0;
  var fabOriginY  = 0;
  var confirmedEl = null;

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

    fab.addEventListener('touchstart',   onTouchStart,  { passive: false });
    document.addEventListener('touchmove',    onTouchMove,   { passive: false });
    document.addEventListener('touchend',     onTouchEnd);
    document.addEventListener('touchcancel',  onTouchEnd);

    overlay && overlay.addEventListener('click', function () {
      if (tapOpen && !isDragging) closeTapMenu();
    });
  }

  // ── Tap mode (short press) ─────────────────────────────────────────────────
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

  // ── Drag mode (long press) ─────────────────────────────────────────────────
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

    // Animate FAB back to origin
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
      setTimeout(function () { trigger.click(); }, 60);
    }
  }

  // ── Touch handlers ─────────────────────────────────────────────────────────
  function onTouchStart(e) {
    e.preventDefault();
    fab.classList.add('touched');

    var rect = fab.getBoundingClientRect();
    var cx   = rect.left + rect.width  / 2;
    var cy   = rect.top  + rect.height / 2;

    pressTimer = setTimeout(function () { openDragMenu(cx, cy); }, LONG_PRESS_MS);
  }

  function onTouchMove(e) {
    var touch = e.touches[0];

    if (!isDragging) {
      // Cancel long press if finger drifted
      if (pressTimer) {
        var rect = fab.getBoundingClientRect();
        var ddx  = touch.clientX - (rect.left + rect.width  / 2);
        var ddy  = touch.clientY - (rect.top  + rect.height / 2);
        if (Math.hypot(ddx, ddy) > 12) {
          clearTimeout(pressTimer);
          pressTimer = null;
          fab.classList.remove('touched');
        }
      }
      return;
    }

    e.preventDefault();

    var rawDx   = touch.clientX - fabOriginX;
    var rawDy   = touch.clientY - fabOriginY;
    var rawDist = Math.hypot(rawDx, rawDy);
    var clamp   = rawDist > MAX_DRAG_RADIUS ? MAX_DRAG_RADIUS / rawDist : 1;
    var movX    = rawDx * clamp;
    var movY    = rawDy * clamp;

    // Visual center of dragged FAB
    var fabCx = fabOriginX + movX;
    var fabCy = fabOriginY + movY;

    // Find closest item
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
      // SNAP — lock on item
      finalX = closest.ix - fabOriginX;
      finalY = closest.iy - fabOriginY;
      fab.style.transition = 'transform 0.12s cubic-bezier(0.34, 1.56, 0.64, 1)';
      setItemState('ready', closest.el);
      showGlassy(closest.el.dataset.label, true);
      confirmedEl = closest.el;

    } else if (closest && closestD < MAGNETIC_DIST) {
      // MAGNETIC pull — partial lerp toward item
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
      // Free drag
      fab.style.transition = 'none';
      setItemState('none', null);
      glassyLabel && glassyLabel.classList.remove('active', 'confirmed');
      confirmedEl = null;
    }

    fab.style.transform = 'translate(' + finalX + 'px,' + finalY + 'px) scale(1.22)';
  }

  function onTouchEnd() {
    clearTimeout(pressTimer);
    pressTimer = null;
    fab.classList.remove('touched');

    if (!isDragging) {
      // Short tap → toggle
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
