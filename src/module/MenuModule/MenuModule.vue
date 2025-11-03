<script setup lang="ts">
import menu from "@/assets/svg/menu.svg";
import IconButton from "@/components/IconButton/index.vue";
import Settings from "@/module/Settings/modal.vue";

const SettingsRef = ref()

const handleClick = (item) => {
  if(item.key === "settings") {
    SettingsRef.value.open();
  }
}
</script>

<template>
  <a-dropdown trigger="click">
    <IconButton class="size-28px!">
      <img class="size-15px" :src="menu" alt="">
    </IconButton>
    <template #overlay>
      <div>
        <a-menu @click="handleClick">
          <a-menu-item  key="settings">
            <SettingOutlined /> 设置
          </a-menu-item>
          <a-menu-item key="exit">
            <SwapRightOutlined /> 退出
          </a-menu-item>
        </a-menu>
      </div>
    </template>
  </a-dropdown>

  <Settings ref="SettingsRef"/>
</template>
