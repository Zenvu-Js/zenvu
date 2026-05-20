import App from './App.Zenvu';

const app = App(document.getElementById('app')!);

// Enable HMR in development
if (import.meta.hot) {
  import.meta.hot.accept();
}
