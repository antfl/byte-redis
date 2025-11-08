<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import { DeleteOutlined, PlusOutlined } from '@ant-design/icons-vue';
import { message } from "ant-design-vue";
import { getTypeColor } from '@/utils/format'

export type KeyType = "string" | "hash" | "list" | "set" | "zset";

interface HashItem {
	field: string;
	value: string;
}

interface ZSetItem {
	score: number;
	value: string;
}

const visible = ref(false);
const loading = ref(false);

const formData = reactive({
	key: "",
	type: "string" as KeyType,
	ttl: -1,
	stringValue: "",
	hashValue: [{ field: "", value: "" }] as HashItem[],
	listValue: [""] as string[],
	setValue: [""] as string[],
	zsetValue: [{ score: 0, value: "" }] as ZSetItem[],
});

let successCallback: ((data: typeof formData) => Promise<void>) | null = null;

// 重置表单状态
const resetForm = () => {
	formData.key = "";
	formData.type = "string";
	formData.ttl = -1;
	formData.stringValue = "";
	formData.hashValue = [{ field: "", value: "" }];
	formData.listValue = [""];
	formData.setValue = [""];
	formData.zsetValue = [{ score: 0, value: "" }];
	loading.value = false;
};

// 当对话框关闭时重置表单
watch(visible, (val) => {
	if (!val) resetForm();
});

const open = (options?: {
	onSuccess?: (data: typeof formData) => Promise<void>;
}) => {
	if (options?.onSuccess) {
		successCallback = options.onSuccess;
	}
	visible.value = true;
};

const handleSubmit = async () => {
	if (!formData.key.trim()) {
		message.error("键名不能为空");
		return;
	}

	// 验证不同类型的数据
	switch (formData.type) {
		case "string":
			if (!formData.stringValue.trim()) {
				message.error("字符串值不能为空");
				return;
			}
			break;
		case "hash":
			for (const item of formData.hashValue) {
				if (!item.field.trim() || !item.value.trim()) {
					message.error("哈希字段名和值不能为空");
					return;
				}
			}
			break;
		case "list":
			for (const item of formData.listValue) {
				if (!item.trim()) {
					message.error("列表元素不能为空");
					return;
				}
			}
			break;
		case "set":
			for (const item of formData.setValue) {
				if (!item.trim()) {
					message.error("集合元素不能为空");
					return;
				}
			}
			break;
		case "zset":
			for (const item of formData.zsetValue) {
				if (!item.value.trim()) {
					message.error("有序集合元素不能为空");
					return;
				}
			}
			break;
	}

	if (!successCallback) return;

	try {
		loading.value = true;
		await successCallback({ ...formData });
		visible.value = false;
		message.success("键添加成功");
	} catch (error) {
		message.error("添加键失败");
	} finally {
		loading.value = false;
	}
};

// 添加哈希字段
const addHashField = () => {
	formData.hashValue.push({ field: "", value: "" });
};

// 删除哈希字段
const removeHashField = (index: number) => {
	if (formData.hashValue.length > 1) {
		formData.hashValue.splice(index, 1);
	} else {
		formData.hashValue[0] = { field: "", value: "" };
	}
};

// 添加列表元素
const addListItem = () => {
	formData.listValue.push("");
};

// 删除列表元素
const removeListItem = (index: number) => {
	if (formData.listValue.length > 1) {
		formData.listValue.splice(index, 1);
	} else {
		formData.listValue[0] = "";
	}
};

// 添加集合元素
const addSetItem = () => {
	formData.setValue.push("");
};

// 删除集合元素
const removeSetItem = (index: number) => {
	if (formData.setValue.length > 1) {
		formData.setValue.splice(index, 1);
	} else {
		formData.setValue[0] = "";
	}
};

// 添加有序集合元素
const addZSetItem = () => {
	formData.zsetValue.push({ score: 0, value: "" });
};

// 删除有序集合元素
const removeZSetItem = (index: number) => {
	if (formData.zsetValue.length > 1) {
		formData.zsetValue.splice(index, 1);
	} else {
		formData.zsetValue[0] = { score: 0, value: "" };
	}
};

defineExpose({ open });
</script>

<template>
  <a-modal
    v-model:open="visible"
    title="添加新键"
    :footer="null"
    width="800px"
    :centered="true"
    class="byte-modal"
    :maskClosable="false"
    :keyboard="false"
    :destroyOnClose="true"
  >
    <a-form layout="vertical">
      <a-row :gutter="24">
        <a-col :span="12">
          <a-form-item label="键名" required>
            <a-input
              v-model:value="formData.key"
              placeholder="输入键名"
              :disabled="loading"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item required>
            <template #label>
              <span>类型</span>
              <a-tag class="ml-8px font-700" :color="getTypeColor(formData.type)" :bordered="false">
                {{ formData.type.toUpperCase() }}
              </a-tag>
            </template>
            <a-select
              v-model:value="formData.type"
              :disabled="loading"
            >
              <a-select-option value="string">字符串</a-select-option>
              <a-select-option value="hash">哈希</a-select-option>
              <a-select-option value="list">列表</a-select-option>
              <a-select-option value="set">集合</a-select-option>
              <a-select-option value="zset">有序集合</a-select-option>
            </a-select>
          </a-form-item>
        </a-col>
      </a-row>

      <!-- 字符串类型 -->
      <div v-if="formData.type === 'string'">
        <a-form-item label="值" required>
          <a-textarea
            v-model:value="formData.stringValue"
            :rows="4"
            placeholder="输入键值"
            :disabled="loading"
          />
        </a-form-item>
      </div>

      <!-- 哈希类型 -->
      <div v-if="formData.type === 'hash'">
        <a-form-item label="哈希字段">
          <div class="hash-fields-container">
            <div v-for="(item, index) in formData.hashValue" :key="index" class="hash-field-item">
              <a-row :gutter="12" align="middle">
                <a-col :span="10">
                  <a-input
                    v-model:value="item.field"
                    placeholder="字段名"
                    :disabled="loading"
                  />
                </a-col>
                <a-col :span="12">
                  <a-input
                    v-model:value="item.value"
                    placeholder="字段值"
                    :disabled="loading"
                  />
                </a-col>
                <a-col :span="2">
                  <a-button
                    type="text"
                    danger
                    @click="removeHashField(index)"
                    :disabled="loading"
                  >
                    <delete-outlined />
                  </a-button>
                </a-col>
              </a-row>
            </div>
            <a-button
              type="dashed"
              @click="addHashField"
              :disabled="loading"
              class="add-field-btn"
            >
              <plus-outlined />
              添加字段
            </a-button>
          </div>
        </a-form-item>
      </div>

      <!-- 列表类型 -->
      <div v-if="formData.type === 'list'">
        <a-form-item label="列表元素">
          <div class="list-items-container">
            <div v-for="(_, index) in formData.listValue" :key="index" class="list-item">
              <a-row :gutter="12" align="middle">
                <a-col :span="22">
                  <a-input
                    v-model:value="formData.listValue[index]"
                    placeholder="元素值"
                    :disabled="loading"
                  />
                </a-col>
                <a-col :span="2">
                  <a-button
                    type="text"
                    danger
                    @click="removeListItem(index)"
                    :disabled="loading"
                  >
                    <delete-outlined />
                  </a-button>
                </a-col>
              </a-row>
            </div>
            <a-button
              type="dashed"
              @click="addListItem"
              :disabled="loading"
              class="add-field-btn"
            >
              <plus-outlined />
              添加元素
            </a-button>
          </div>
        </a-form-item>
      </div>

      <!-- 集合类型 -->
      <div v-if="formData.type === 'set'">
        <a-form-item label="集合元素">
          <div class="set-items-container">
            <div v-for="(_, index) in formData.setValue" :key="index" class="set-item">
              <a-row :gutter="12" align="middle">
                <a-col :span="22">
                  <a-input
                    v-model:value="formData.setValue[index]"
                    placeholder="元素值"
                    :disabled="loading"
                  />
                </a-col>
                <a-col :span="2">
                  <a-button
                    type="text"
                    danger
                    @click="removeSetItem(index)"
                    :disabled="loading"
                  >
                    <delete-outlined />
                  </a-button>
                </a-col>
              </a-row>
            </div>
            <a-button
              type="dashed"
              @click="addSetItem"
              :disabled="loading"
              class="add-field-btn"
            >
              <plus-outlined />
              添加元素
            </a-button>
          </div>
        </a-form-item>
      </div>

      <!-- 有序集合类型 -->
      <div v-if="formData.type === 'zset'">
        <a-form-item label="有序集合元素" :help="'分数 (Score) 用于排序，分数越小越靠前。可以是任意浮点数（包括负数）'">
          <div class="zset-items-container">
            <div v-for="(item, index) in formData.zsetValue" :key="index" class="zset-item">
              <a-row :gutter="12" align="middle">
                <a-col :span="8">
                  <a-input-number
                    v-model:value="item.score"
                    placeholder="分数 (Score)"
                    :precision="2"
                    :step="0.1"
                    :disabled="loading"
                    class="w-full"
                  />
                </a-col>
                <a-col :span="14">
                  <a-input
                    v-model:value="item.value"
                    placeholder="元素值"
                    :disabled="loading"
                  />
                </a-col>
                <a-col :span="2">
                  <a-button
                    type="text"
                    danger
                    @click="removeZSetItem(index)"
                    :disabled="loading"
                  >
                    <delete-outlined />
                  </a-button>
                </a-col>
              </a-row>
            </div>
            <a-button
              type="dashed"
              @click="addZSetItem"
              :disabled="loading"
              class="add-field-btn"
            >
              <plus-outlined />
              添加元素
            </a-button>
          </div>
        </a-form-item>
      </div>

      <a-form-item>
        <template #label>
          <span>过期时间 (秒)</span>
          <a-tag class="ml-16px" color="blue" :bordered="false">-1 表示永不过期</a-tag>
        </template>
        <a-input-number
          class="w-200px"
          v-model:value="formData.ttl"
          :min="-1"
          :step="1"
          :disabled="loading"
        />
      </a-form-item>

      <div class="flex justify-end gap-10px">
        <a-button
          @click="visible = false"
          :disabled="loading"
        >
          取消
        </a-button>
        <a-button
          type="primary"
          @click="handleSubmit"
          :loading="loading"
        >
          添加
        </a-button>
      </div>
    </a-form>
  </a-modal>
</template>

<style scoped>
.hash-fields-container,
.list-items-container,
.set-items-container,
.zset-items-container {
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  padding: 12px;
  background-color: #fafafa;
}

.hash-field-item,
.list-item,
.set-item,
.zset-item {
  margin-bottom: 12px;
}

.hash-field-item:last-child,
.list-item:last-child,
.set-item:last-child,
.zset-item:last-child {
  margin-bottom: 0;
}

.add-field-btn {
  width: 100%;
  margin-top: 12px;
}

.w-full {
  width: 100%;
}
</style>

