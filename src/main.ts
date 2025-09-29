import { createApp } from "vue";
import App from "./App.vue";

import "virtual:uno.css";
import "@/assets/style/common.less";
import "@/assets/style/theme.less";
import "@/assets/style/antd.less";

const app = createApp(App);

app.mount("#app");
