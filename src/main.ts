import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";

const pinia = createPinia();

import "virtual:uno.css";
import "@/assets/style/common.less";
import "@/assets/style/theme.less";
import "@/assets/style/antd.less";

const app = createApp(App);
app.use(pinia);
app.use(router);

app.mount("#app");
