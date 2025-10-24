<script setup lang="ts">
import ServerInfo from "@/module/ServerInfo/index.vue";

const ServerInfoRef = ref();
const openChange = (v) => {
	console.log(v);
	if (v) {
		nextTick(() => {
			ServerInfoRef.value.load();
		});
	}
};
</script>

<template>
  <div class="view-footer h-[var(--view-footer-height)] bg-#fff">
    <div class="px-16px h-full flex items-center justify-between">
      <div class="cursor-pointer">
        <a-badge status="success"/>
        <span class="font-size-13px">已连接</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.view-footer {
  border-top: 1px solid #f0f0f0;
}
</style>
