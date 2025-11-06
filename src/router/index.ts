import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
	{ path: "/", redirect: "/data" },
	{ path: "/data", name: "RedisData", component: () => import("@/module/RedisData/index.vue") },
	{ path: "/stats", name: "RedisStats", component: () => import("@/module/RedisStats/index.vue") },
];

export const router = createRouter({
	history: createWebHashHistory(),
	routes,
});

export default router;


