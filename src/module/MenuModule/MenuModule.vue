<script setup lang="ts">
import menu from "@/assets/svg/menu.svg";
</script>

<template>
  <a-dropdown>
    <img class="size-15px" :src="menu" alt="">
    <template #overlay>
      <div>
        <a-menu>
          <a-menu-item>
            主题
          </a-menu-item>
          <a-menu-item>
            设置
          </a-menu-item>
          <a-menu-item>
            退出
          </a-menu-item>
        </a-menu>
      </div>
    </template>
  </a-dropdown>
</template>

<style scoped>

</style>
