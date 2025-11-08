import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Ref, ComputedRef } from "vue";

export interface Connection {
	id: string;
	name: string;
	host: string;
	port: number;
	username?: string | null;
	password?: string | null;
	db?: number | null;
	separator?: string | null;
}

export const useConnectionStore = defineStore("connection", () => {
	const connections: Ref<Connection[]> = ref([]);
	const activeConnectionId: Ref<string | null> = ref(null);
	const currentDbIndex: Ref<number> = ref(0);
	const trigger: Ref<number> = ref(0);
	const keyListRefreshTrigger: Ref<number> = ref(0);

	const currentKey = ref<string | null>(null);
	const currentKeyCount = ref<number>(0);

	const activeConnection: ComputedRef<Connection | null> = computed(() => {
		if (!activeConnectionId.value) return null;
		return (
			connections.value.find((c) => c.id === activeConnectionId.value) || null
		);
	});

	const connectionCount: ComputedRef<number> = computed(
		() => connections.value.length,
	);

	const createConnection = (data: Omit<Connection, "id">): void => {
		const newConnection: Connection = {
			...data,
			separator: data.separator ?? ":",
			id: generateUniqueId(),
		};

		connections.value.push(newConnection);
		trigger.value++;
		setActiveConnection(newConnection.id);
	};

	const updateConnection = (
		id: string,
		data: Partial<Omit<Connection, "id">>,
	): void => {
		const index = connections.value.findIndex((c) => c.id === id);
		if (index !== -1) {
			connections.value[index] = {
				...connections.value[index],
				...data,
				separator: data.separator ?? connections.value[index].separator ?? ":",
			};
			trigger.value++;
			saveToLocalStorage();
		}
	};

	const deleteConnection = (id: string): void => {
		const index = connections.value.findIndex((c) => c.id === id);
		if (index !== -1) {
			connections.value.splice(index, 1);

			if (activeConnectionId.value === id) {
				activeConnectionId.value = null;
			}

			trigger.value++;
			saveToLocalStorage();
		}
	};

	const setCurrentKey = (key: string | null) => {
		currentKey.value = key;
	};

	const setCurrentKeyCount = (count: number) => {
		currentKeyCount.value = count;
	};

	const resetKeyState = () => {
		currentKey.value = null;
		currentKeyCount.value = 0;
	};

	const setActiveConnection = (id: string | null): void => {
		if (id && !connections.value.some((c) => c.id === id)) {
			console.warn(`连接ID ${id} 不存在`);
			return;
		}

		activeConnectionId.value = id;
		// 重置数据库索引为连接配置的默认数据库或 0
		if (id) {
			const conn = connections.value.find((c) => c.id === id);
			currentDbIndex.value = conn?.db ?? 0;
		} else {
			currentDbIndex.value = 0;
		}
		resetKeyState();
		trigger.value++;
		saveToLocalStorage();
	};

	const setCurrentDbIndex = (dbIndex: number): void => {
		currentDbIndex.value = dbIndex;
		trigger.value++;
		saveToLocalStorage();
	};

	const notify = (): void => {
		trigger.value++;
	};

	const refreshKeyList = (): void => {
		keyListRefreshTrigger.value++;
	};

	const generateUniqueId = (): string => {
		return Date.now().toString(36) + Math.random().toString(36).substring(2);
	};

	const STORAGE_KEY = "redis-connections";

	const saveToLocalStorage = (): void => {
		const stateToSave = {
			connections: connections.value,
			activeConnectionId: activeConnectionId.value,
			currentDbIndex: currentDbIndex.value,
		};
		localStorage.setItem(STORAGE_KEY, JSON.stringify(stateToSave));
	};

	const loadFromLocalStorage = (): void => {
		const savedData = localStorage.getItem(STORAGE_KEY);
		if (savedData) {
			try {
				const parsedData = JSON.parse(savedData);
				if (parsedData.connections) {
					connections.value = parsedData.connections.map((connection: Connection) => ({
						...connection,
						separator: connection.separator ?? ":",
					}));
				}
				if (parsedData.activeConnectionId) {
					activeConnectionId.value = parsedData.activeConnectionId;
				}
				if (parsedData.currentDbIndex !== undefined) {
					currentDbIndex.value = parsedData.currentDbIndex;
				}

				if (
					activeConnectionId.value &&
					!connections.value.some((c) => c.id === activeConnectionId.value)
				) {
					activeConnectionId.value = null;
				}
			} catch (e) {
				console.error("Failed to load state from localStorage", e);
			}
		}
	};

	loadFromLocalStorage();

	return {
		currentKey,
		currentKeyCount,
		connections,
		activeConnectionId,
		currentDbIndex,
		trigger,
		keyListRefreshTrigger,

		activeConnection,
		connectionCount,

		createConnection,
		updateConnection,
		deleteConnection,
		setActiveConnection,
		setCurrentDbIndex,
		notify,
		refreshKeyList,

		saveToLocalStorage,
		loadFromLocalStorage,
		setCurrentKey,
		setCurrentKeyCount,
		resetKeyState,
	};
});
