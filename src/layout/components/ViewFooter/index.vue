<script setup lang="ts">
import ConnectionStatusInfo from "@/module/Connection/components/ConnectionStatusInfo.vue";
</script>

<template>
  <div class="view-footer h-[var(--view-footer-height)] bg-[var(--color-bg-container)]">
    <div class="px-16px h-full flex items-center justify-between">
      <ConnectionStatusInfo />
    </div>
  </div>
</template>

<style scoped lang="less">
.view-footer {
  border-top: 1px solid var(--color-border-secondary);
}
</style>
