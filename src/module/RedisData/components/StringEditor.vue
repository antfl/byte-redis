<template>
  <div class="value-editor">
    <a-textarea
      v-model:value="localKeyData.value"
      placeholder="值内容"
      :auto-size="{ minRows: 4, maxRows: 10 }"
    />
  </div>
</template>

<script setup lang="ts">
import { toRefs } from "vue";

interface RedisKey {
	key: string;
	type: string;
	ttl: number;
	size: number;
	value: any;
}

const props = defineProps<{ keyData: RedisKey }>();
const { keyData: localKeyData } = toRefs(props);
</script>

<style scoped>
.value-editor {
  margin-bottom: 16px;
}
</style>


