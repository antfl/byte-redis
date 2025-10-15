<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="flex items-center">
      <div class="app-info">
        <img class="app-logo" :src="Logo" :alt="appName">
      </div>
      <a-dropdown>
        <img class="size-16px" :src="menu" alt="">
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
      <a-dropdown class="ml-20px">
        <div class="flex items-center">
          <div class="leading-0px">
            <a-badge status="success"/>
          </div>
          <a-tag :bordered="false">
            本地连接
          </a-tag>
          <img class="size-10px" :src="downOutlined" alt="">
        </div>
        <template #overlay>
          <div>
            <a-menu>
              <a-menu-item>
                <PlusOutlined/>
                <span class="ml-5px">新建连接</span>
              </a-menu-item>
              <a-menu-item>
                localhost
              </a-menu-item>
              <a-menu-item>
                adminx
              </a-menu-item>
              <a-menu-item>
                本地连接
              </a-menu-item>
              <a-menu-item>
                127.0.0.1
              </a-menu-item>
            </a-menu>
          </div>
        </template>
      </a-dropdown>
      <a-dropdown class="ml-20px">
        <div class="flex items-center">
          <img class="size-24px" :src="database" alt="">
          <a-tag class="ml-5px" :bordered="false">
            DB01
          </a-tag>
          <img class="size-10px" :src="downOutlined" alt="">
        </div>
        <template #overlay>
          <a-menu>
            <a-menu-item>
              <a-tag :bordered="false">DB01</a-tag>
              <span class="ml-36px">11</span>
            </a-menu-item>
            <a-menu-item>
              <a-tag :bordered="false">DB02</a-tag>
              <span class="ml-36px">11</span>
            </a-menu-item>
            <a-menu-item>
              <a-tag :bordered="false">DB03</a-tag>
              <span class="ml-36px">11</span>
            </a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
    </div>

    <div class="window-controls">
      <div class="control-btn" @click="minimize" title="最小化">
        <MinusOutlined/>
      </div>
      <div class="control-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <img class="size-14px" v-if="isMaximized" :src="Restore" alt="">
        <component v-else :is="BorderOutlined"/>
      </div>
      <div class="control-btn close-btn" @click="close" title="关闭">
        <CloseOutlined/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {Window} from "@tauri-apps/api/window";
import {
  MinusOutlined,
  BorderOutlined,
  CloseOutlined,
} from "@ant-design/icons-vue";
import Logo from "@/assets/images/logo.jpg";
import Restore from "@/assets/svg/restore.svg";
import downOutlined from "@/assets/svg/down-outlined.svg";
import database from "@/assets/svg/database.svg";
import menu from "@/assets/svg/menu.svg";

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

<style scoped>
.title-bar {
  height: var(--title-bar-height);
  background-color: #fff;
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
  border-bottom: 1px solid #f0f0f0;
}

.app-info {
  padding-left: 12px;
  margin-right: 20px;
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
}

.app-logo {
  width: 20px;
  height: 20px;
  border-radius: 6px;
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
  .app-name {
    display: none;
  }

  .control-btn {
    width: 28px;
    height: 28px;
  }
}
</style>
