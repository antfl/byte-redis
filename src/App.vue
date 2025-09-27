<script setup>
import zhCN from "ant-design-vue/es/locale/zh_CN";
import Layout from "@/layout/Layout.vue";

const themeConfig = {
	token: {
		borderRadius: 1,
	},
};
</script>

<template>
  <a-config-provider :theme="themeConfig" :locale="zhCN">
    <Layout/>
  </a-config-provider>
</template>

