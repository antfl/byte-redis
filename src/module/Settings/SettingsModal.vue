<script setup lang="ts">
import { ref } from 'vue';
import Theme from './Theme.vue';

const visible = ref(false);

const open = () => {
  visible.value = true;
}

const activeKey = ref('theme')

defineExpose({ open });
</script>

<template>
  <a-modal
    v-model:open="visible"
    title="设置"
    :footer="null"
    width="700px"
    :centered="true"
    :mask="false"
    class="byte-modal"
    :maskClosable="false"
    :keyboard="false"
    :destroyOnClose="true"
  >
    <div class="h-50vh">
      <a-tabs tabPosition="left" v-model:activeKey="activeKey">
        <a-tab-pane key="theme" tab="主题">
          <Theme/>
        </a-tab-pane>
      </a-tabs>
    </div>
  </a-modal>
</template>

