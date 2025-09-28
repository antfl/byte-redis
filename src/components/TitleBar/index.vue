<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="app-info">
      <img class="app-logo" :src="Logo" :alt="appName">
      <div class="app-name">{{ appName }}</div>
    </div>

    <div class="window-controls">
      <div class="control-btn" @click="minimize" title="最小化">
        <MinusOutlined />
      </div>
      <div class="control-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <component :is="isMaximized ? ShrinkOutlined : BorderOutlined" />
      </div>
      <div class="control-btn close-btn" @click="close" title="关闭">
        <CloseOutlined />
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
	ShrinkOutlined,
} from "@ant-design/icons-vue";
import Logo from "@/assets/images/logo.jpg";

const appName = import.meta.env.VITE_APP_NAME || "ByteRedis";

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

<style scoped>
.title-bar {
  height: 40px;
  background: #fff;
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}

.app-info {
  margin-left: 12px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.app-logo {
  width: 24px;
  height: 24px;
  border-radius: 6px;
  object-fit: cover;
}

.app-name {
  font-size: 14px;
  font-weight: 500;
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
  .app-name {
    display: none;
  }

  .control-btn {
    width: 28px;
    height: 28px;
  }
}
</style>
