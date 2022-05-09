import App from './App.svelte';

function appHeight() {
  const doc = document.documentElement;
  doc.style.setProperty('--vh', window.innerHeight + 'px');
}

window.addEventListener('resize', appHeight);
appHeight();

const app = new App({
  target: document.body,
});

export default app;
