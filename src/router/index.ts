import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
	{ path: "/", redirect: "/data" },
	{ path: "/data", name: "RedisData", component: () => import("@/module/RedisData/RedisData.vue") },
	{ path: "/stats", name: "RedisStats", component: () => import("@/module/RedisStats/RedisStats.vue") },
];

export const router = createRouter({
	history: createWebHashHistory(),
	routes,
});

export default router;


