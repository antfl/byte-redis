<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="flex items-center">
      <div class="app-info">
        <img class="app-logo" :src="Logo" :alt="appName">
      </div>

      <!-- 菜单模块 -->
      <MenuModule/>

      <!-- 连接管理 -->
      <ConnectionManager/>

      <!-- 数据库管理 -->
      <DatabaseManager/>
    </div>

    <div class="window-controls">
      <div class="control-btn" @click="minimize" title="最小化">
        <MinusOutlined/>
      </div>
      <div class="control-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <img class="size-12px" v-if="isMaximized" :src="Restore" alt="">
        <component v-else :is="BorderOutlined"/>
      </div>
      <div class="control-btn close-btn" @click="close" title="关闭">
        <CloseOutlined/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Window } from "@tauri-apps/api/window";
import {
	MinusOutlined,
	BorderOutlined,
	CloseOutlined,
} from "@ant-design/icons-vue";
import Logo from "@/assets/images/logo.jpg";
import Restore from "@/assets/svg/restore.svg";
import ConnectionManager from "@/module/ConnectionModule/ConnectionManager.vue";
import DatabaseManager from "@/module/DatabaseModule/DatabaseManager.vue";
import MenuModule from "@/module/MenuModule/MenuModule.vue";

const appName = import.meta.env.VITE_APP_NAME;

const appWindow = new Window("main");

// 窗口状态
const isMaximized = ref(false);

// 初始化检查窗口状态
onMounted(async () => {
	isMaximized.value = await appWindow.isMaximized();
});

// 监听窗口状态变化
appWindow.onResized(async () => {
	isMaximized.value = await appWindow.isMaximized();
});

/** 窗口最小化 */
const minimize = () => {
	appWindow.minimize();
};

/** 切换最大化和还原 */
const toggleMaximize = () => {
	appWindow.toggleMaximize();
};

/** 关闭窗口 */
const close = () => {
	appWindow.close();
};
</script>

<style scoped lang="less">
.title-bar {
  height: var(--title-bar-height);
  background-color: var(--color-bg-container);
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  overflow: hidden;
  border-bottom: 1px solid var(--color-border-secondary);
}

.app-info {
  padding-left: 12px;
  margin-right: 24px;
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
}

.app-logo {
  width: 20px;
  height: 20px;
  border-radius: 3px;
  object-fit: cover;
}

.window-controls {
  display: flex;
  align-items: center;
}

.control-btn {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #2b2d30;
}

.control-btn:hover {
  background-color: #f0f0f0;
}

.close-btn:hover {
  background-color: #ff4d4f;
  color: white;
}

@media (max-width: 600px) {
  .control-btn {
    width: 28px;
    height: 28px;
  }
}
</style>
