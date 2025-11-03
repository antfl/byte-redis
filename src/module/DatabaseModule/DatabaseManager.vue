<script setup lang="ts">
import database from "@/assets/svg/database.svg";
import downOutlined from "@/assets/svg/down-outlined.svg";
import IconButton from "@/components/IconButton/index.vue";

</script>

<template>
  <a-dropdown class="ml-24px">
    <IconButton>
      <img class="size-24px" :src="database" alt="">
      <div class="mr-5px">
        DB01
      </div>
      <img class="size-10px" :src="downOutlined" alt="">
    </IconButton>
    <template #overlay>
      <a-menu>
        <a-menu-item>
          <span class="color-#1e1f22">DB01</span>
          <span class="ml-8px">[11]</span>
        </a-menu-item>
        <a-menu-item>
          <span class="color-#1e1f22">DB02</span>
          <span class="ml-8px">[112]</span>
        </a-menu-item>
        <a-menu-item>
          <span class="color-#1e1f22">DB03</span>
          <span class="ml-8px">[121]</span>
        </a-menu-item>
      </a-menu>
    </template>
  </a-dropdown>
</template>
