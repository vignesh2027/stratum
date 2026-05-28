'use strict';

// ── Animated query demo in hero ──────────────────────────────────────────────
function runQueryDemo() {
  const lines  = document.querySelectorAll('.qline');
  const result = document.getElementById('qresult');
  let delay = 350;

  lines.forEach((line, i) => {
    setTimeout(() => line.classList.add('show'), delay + i * 160);
  });
  setTimeout(() => result && result.classList.add('show'), delay + lines.length * 160 + 280);
}

// ── Nav scroll shadow ────────────────────────────────────────────────────────
function setupNav() {
  const nav = document.getElementById('nav');
  if (!nav) return;
  window.addEventListener('scroll', () => {
    nav.style.boxShadow = window.scrollY > 8
      ? '0 1px 12px rgba(0,0,0,0.08)'
      : '0 1px 0 rgba(0,0,0,0.04)';
  }, { passive: true });
}

// ── Scroll-in animations ─────────────────────────────────────────────────────
function setupScrollAnimations() {
  const targets = document.querySelectorAll(
    '.feat, .how-card, .usecase-card, .sqsl-ex, .api-ex, .how-card, .arch-box, .comparison-table'
  );

  const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');
        observer.unobserve(entry.target);
      }
    });
  }, { threshold: 0.08, rootMargin: '0px 0px -32px 0px' });

  targets.forEach((el, i) => {
    el.classList.add('animate-in');
    el.style.transitionDelay = `${(i % 4) * 70}ms`;
    observer.observe(el);
  });
}

// ── Active nav link on scroll ─────────────────────────────────────────────────
function setupActiveNav() {
  const sections = document.querySelectorAll('section[id], header[id]');
  const links    = document.querySelectorAll('.nav-links a[href^="#"]');

  const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        links.forEach(link => {
          const active = link.getAttribute('href') === `#${entry.target.id}`;
          link.style.color = active ? 'var(--green-700)' : '';
          link.style.fontWeight = active ? '600' : '';
        });
      }
    });
  }, { threshold: 0.4 });

  sections.forEach(s => observer.observe(s));
}

// ── Copy buttons on code blocks ───────────────────────────────────────────────
function setupCopyButtons() {
  document.querySelectorAll('pre').forEach(pre => {
    if (pre.parentElement.tagName === 'DIV' && !pre.querySelector('button')) {
      const btn = document.createElement('button');
      btn.textContent = 'Copy';
      btn.style.cssText = [
        'position:absolute', 'top:10px', 'right:12px',
        'padding:3px 10px', 'font-size:11px', 'font-weight:600',
        'font-family:var(--font-sans)',
        'background:rgba(255,255,255,0.08)', 'border:1px solid rgba(255,255,255,0.12)',
        'border-radius:4px', 'color:rgba(255,255,255,0.45)',
        'cursor:pointer', 'transition:all 0.15s',
      ].join(';');

      btn.addEventListener('mouseenter', () => {
        btn.style.background = 'rgba(34,197,94,0.15)';
        btn.style.color = '#4ade80';
        btn.style.borderColor = 'rgba(34,197,94,0.3)';
      });
      btn.addEventListener('mouseleave', () => {
        btn.style.background = 'rgba(255,255,255,0.08)';
        btn.style.color = 'rgba(255,255,255,0.45)';
        btn.style.borderColor = 'rgba(255,255,255,0.12)';
      });
      btn.addEventListener('click', () => {
        navigator.clipboard.writeText(pre.innerText).then(() => {
          btn.textContent = 'Copied!';
          btn.style.color = '#4ade80';
          setTimeout(() => {
            btn.textContent = 'Copy';
            btn.style.color = 'rgba(255,255,255,0.45)';
          }, 1800);
        });
      });
      pre.style.position = 'relative';
      pre.appendChild(btn);
    }
  });
}

// ── Subtle particle field in hero ornament ────────────────────────────────────
function createParticles() {
  const hero = document.querySelector('.hero');
  if (!hero) return;
  for (let i = 0; i < 14; i++) {
    const p = document.createElement('div');
    const size = Math.random() * 4 + 2;
    p.style.cssText = [
      'position:absolute',
      `width:${size}px`, `height:${size}px`,
      'border-radius:50%',
      `background:rgba(22,163,74,${Math.random() * 0.15 + 0.04})`,
      `left:${Math.random() * 100}%`,
      `top:${Math.random() * 100}%`,
      `animation:pip-pulse ${5 + Math.random() * 7}s ease-in-out ${Math.random() * 5}s infinite`,
      'pointer-events:none', 'z-index:0',
    ].join(';');
    hero.appendChild(p);
  }
}

// ── Comparison table row hover glow ──────────────────────────────────────────
function setupTableHighlight() {
  document.querySelectorAll('.comparison-table tbody tr:not(.highlight-row)').forEach(row => {
    row.addEventListener('mouseenter', () => row.style.background = 'var(--green-50)');
    row.addEventListener('mouseleave', () => row.style.background = '');
  });
}

// ── Init ──────────────────────────────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', () => {
  runQueryDemo();
  setupNav();
  setupScrollAnimations();
  setupActiveNav();
  setupCopyButtons();
  createParticles();
  setupTableHighlight();
});
