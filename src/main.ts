import './styles.css';
import App from './App.vue';
import { createApp } from 'vue';
import { setupI18n } from './i18n';

const i18n = setupI18n();
createApp(App).use(i18n).mount('#app');
