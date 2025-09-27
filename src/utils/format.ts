/**
 * 格式化 TTL
 */
export const formatTTL = (seconds: number) => {
	if (seconds === -1) return "永不过期";

	const days = Math.floor(seconds / 86400);
	const hours = Math.floor((seconds % 86400) / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;

	let result = "";
	if (days > 0) result += `${days}天 `;
	if (hours > 0) result += `${hours}小时 `;
	if (minutes > 0) result += `${minutes}分 `;
	if (secs > 0 || result === "") result += `${secs}秒`;

	return result;
};
