export interface MenuItemProps {
	id: string | number;
	position: "before" | "after";
	tooltip?: string;
	icon: Component;
}
