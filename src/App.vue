<script setup>
import zhCN from "ant-design-vue/es/locale/zh_CN";
import Layout from "@/layout/Layout.vue";

const themeConfig = {
	token: {
		borderRadius: 2,
	},
};
</script>

<template>
  <div class="app-container">
    <a-config-provider :theme="themeConfig" :locale="zhCN">
      <Layout/>
    </a-config-provider>
  </div>
</template>

<style>
.app-container {
  min-width: 800px;
  min-height: 600px;
  overflow: hidden;
}
</style>

