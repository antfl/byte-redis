<script setup lang="ts">
import menu from "@/assets/svg/menu.svg";
import IconButton from "@/components/IconButton/index.vue"
</script>

<template>
  <a-dropdown trigger="click">
    <IconButton class="size-28px!">
      <img class="size-15px" :src="menu" alt="">
    </IconButton>
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
