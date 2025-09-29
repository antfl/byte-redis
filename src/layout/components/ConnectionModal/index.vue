<script setup lang="ts">
import { message } from "ant-design-vue";
import type { FormInstance } from "ant-design-vue";

// 定义表单状态类型
interface ConnectionFormState {
	id: number | null;
	name: string;
	host: string;
	port: number;
	username: string;
	password: string;
	db: number;
}

// 定义 use 函数的参数类型
interface UseParams {
	onSuccess: (formData: ConnectionFormState) => Promise<void> | void;
	formData: Partial<ConnectionFormState>;
	title: string;
}

const visible = ref(false);
const modalTitle = ref("");

// 初始化表单状态
const formState = ref<ConnectionFormState>({
	id: null,
	name: "",
	host: "127.0.0.1",
	port: 6379,
	username: "",
	password: "",
	db: 0,
});

// 表单引用
const formRef = ref<FormInstance>();

// 关闭模态框
const handleCancel = () => {
	formRef.value?.resetFields();
	visible.value = false;
};

// 提交表单
const handleSubmit = async () => {
	if (!formState.value.name || !formState.value.host) {
		message.warning("请填写连接名称和主机地址");
		return;
	}

	if (callback) {
		await callback(formState.value);
	}

	handleCancel();
};

// 回调函数类型
let callback: ((formData: ConnectionFormState) => Promise<void> | void) | null =
	null;

const use = (params: UseParams) => {
	callback = params.onSuccess;
	modalTitle.value = params.title;

	// 合并表单数据
	formState.value = {
		...formState.value,
		...params.formData,
	};

	visible.value = true;
};

defineExpose({ use });
</script>

<template>
  <a-modal
    class="byte-modal"
    v-model:open="visible"
    :title="modalTitle"
    width="600px"
    :centered="true"
    :mask="false"
    :footer="null"
    :maskClosable="false"
  >
    <a-form ref="formRef" layout="vertical" :model="formState">
      <a-form-item label="连接名称" name="name" required>
        <a-input v-model:value="formState.name" placeholder="连接名称"/>
      </a-form-item>
      <a-form-item label="主机" name="host" required>
        <a-input v-model:value="formState.host" placeholder="主机地址"/>
      </a-form-item>
      <a-form-item label="端口" name="port" required>
        <a-input-number v-model:value="formState.port" :min="1" :max="65535"/>
      </a-form-item>
      <a-form-item label="用户名" name="username">
        <a-input v-model:value="formState.username" placeholder="用户名（可选）"/>
      </a-form-item>
      <a-form-item label="密码" name="password">
        <a-input-password v-model:value="formState.password" placeholder="密码"/>
      </a-form-item>
      <a-form-item label="数据库" name="db">
        <a-input-number v-model:value="formState.db" :min="0"/>
      </a-form-item>
      <div class="flex gap-10px justify-end">
        <a-button @click="handleCancel">取消</a-button>
        <a-button type="primary" @click="handleSubmit">保存</a-button>
      </div>
    </a-form>
  </a-modal>
</template>
