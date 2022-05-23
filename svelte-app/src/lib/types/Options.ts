export interface Option<T> {
	default: T;
	from: T;
	to: T;
	step: number;
}

export type Options<T> = { [Property in keyof T]: Option<T[Property]> };
