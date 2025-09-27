class StorageStore {
	constructor() {}

	get(key: string) {
		const item = localStorage.getItem(key);
		if (item === null) return null;

		try {
			return JSON.parse(item);
		} catch (error) {
			console.error(`解析错误 (key: ${key}):`, error);
			return item;
		}
	}

	set(key: string, value: unknown) {
		try {
			const serialized =
				typeof value === "string" ? value : JSON.stringify(value);
			localStorage.setItem(key, serialized);
			return true;
		} catch (error) {
			console.error(`存储错误 (key: ${key}):`, error);
			return false;
		}
	}

	delete(key: string) {
		if (!localStorage.getItem(key)) return false;
		localStorage.removeItem(key);
		return true;
	}

	clear() {
		localStorage.clear();
	}

	keys() {
		return Object.keys(localStorage);
	}

	values() {
		return Object.keys(localStorage).map((key) => this.get(key));
	}
}

export const store = new StorageStore();
