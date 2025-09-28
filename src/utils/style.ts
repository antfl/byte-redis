export const getLessVar = (name: string) => {
	return getComputedStyle(document.documentElement)
		.getPropertyValue(name)
		.trim();
};
