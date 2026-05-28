'use strict';

// Animate query lines in the hero demo
function animateDemo() {
  const lines = document.querySelectorAll('.query-line');
  const result = document.getElementById('qr');
  let delay = 400;

  lines.forEach((line, i) => {
    setTimeout(() => line.classList.add('visible'), delay + i * 180);
  });
  setTimeout(() => result && result.classList.add('visible'), delay + lines.length * 180 + 300);
}

// Intersection Observer for section animations
function setupScrollAnimations() {
  const cards = document.querySelectorAll(
    '.feature-card, .problem-card, .aqsl-card, .usecase, .step, .arch-layer'
  );

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          entry.target.style.opacity = '1';
          entry.target.style.transform = 'translateY(0)';
          observer.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.1, rootMargin: '0px 0px -40px 0px' }
  );

  cards.forEach((card, i) => {
    card.style.opacity = '0';
    card.style.transform = 'translateY(24px)';
    card.style.transition = `opacity 0.5s ease ${i % 6 * 80}ms, transform 0.5s ease ${i % 6 * 80}ms`;
    observer.observe(card);
  });
}

// Smooth nav active state
function setupNavHighlight() {
  const sections = document.querySelectorAll('section[id]');
  const navLinks = document.querySelectorAll('.nav-links a[href^="#"]');

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          navLinks.forEach(link => {
            link.style.color = '';
            if (link.getAttribute('href') === `#${entry.target.id}`) {
              link.style.color = 'var(--teal-400)';
            }
          });
        }
      });
    },
    { threshold: 0.5 }
  );

  sections.forEach(s => observer.observe(s));
}

// Code block syntax highlighting for plain <pre> blocks without .kw spans
// (only applies additional polish on load)
function polishCodeBlocks() {
  const pres = document.querySelectorAll('pre.code-block');
  pres.forEach(pre => {
    if (!pre.querySelector('.kw')) {
      let html = pre.innerHTML;
      // Highlight AQSL keywords in unformatted blocks
      const keywords = ['FIND', 'records', 'WHERE', 'AND', 'OR', 'ORDER BY', 'LIMIT', 'OFFSET',
                        'BETWEEN', 'AFTER', 'BEFORE', 'WITH', 'threshold', 'depth', 'DESC', 'ASC',
                        'caused_by', 'similar_to', 'embedding', 'time'];
      keywords.forEach(kw => {
        const re = new RegExp(`\\b(${kw})\\b`, 'g');
        html = html.replace(re, `<span class="kw">$1</span>`);
      });
      pre.innerHTML = html;
    }
  });
}

// Copy-to-clipboard for code blocks
function setupCopyButtons() {
  document.querySelectorAll('pre.code-block').forEach(pre => {
    const btn = document.createElement('button');
    btn.textContent = 'Copy';
    btn.style.cssText = `
      position: absolute; top: 10px; right: 12px;
      padding: 4px 10px; font-size: 11px; font-weight: 600;
      background: rgba(255,255,255,0.06); border: 1px solid rgba(255,255,255,0.1);
      border-radius: 4px; color: #94a3b8; cursor: pointer;
      transition: all 0.15s; font-family: var(--font-sans);
    `;
    btn.addEventListener('mouseenter', () => {
      btn.style.background = 'rgba(13,148,136,0.2)';
      btn.style.color = '#5eead4';
    });
    btn.addEventListener('mouseleave', () => {
      btn.style.background = 'rgba(255,255,255,0.06)';
      btn.style.color = '#94a3b8';
    });
    btn.addEventListener('click', () => {
      navigator.clipboard.writeText(pre.innerText).then(() => {
        btn.textContent = 'Copied!';
        btn.style.color = '#5eead4';
        setTimeout(() => { btn.textContent = 'Copy'; btn.style.color = '#94a3b8'; }, 1800);
      });
    });
    pre.style.position = 'relative';
    pre.appendChild(btn);
  });
}

// Staggered particle dots in hero background
function createParticles() {
  const bg = document.querySelector('.hero-bg');
  if (!bg) return;
  for (let i = 0; i < 18; i++) {
    const dot = document.createElement('div');
    const size = Math.random() * 3 + 1;
    dot.style.cssText = `
      position: absolute;
      width: ${size}px; height: ${size}px;
      border-radius: 50%;
      background: rgba(13,148,136,${Math.random() * 0.4 + 0.1});
      left: ${Math.random() * 100}%;
      top: ${Math.random() * 100}%;
      animation: float ${6 + Math.random() * 8}s ease-in-out ${Math.random() * 6}s infinite;
      pointer-events: none;
    `;
    bg.appendChild(dot);
  }
}

document.addEventListener('DOMContentLoaded', () => {
  animateDemo();
  setupScrollAnimations();
  setupNavHighlight();
  setupCopyButtons();
  createParticles();
});
