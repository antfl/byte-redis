<template>
  <div class="view-footer h-[var(--view-footer-height)] bg-[var(--color-bg-container)]">
    <div class="px-16px h-full flex items-center justify-between">
      <div class="cursor-pointer">
        <a-badge status="success"/>
        <span class="font-size-13px">已连接</span>
      </div>
    </div>
  </div>
</template>

<style scoped lang="less">
.view-footer {
  border-top: 1px solid var(--color-border-secondary);
}
</style>
