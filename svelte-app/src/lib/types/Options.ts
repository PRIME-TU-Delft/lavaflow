export interface RangeOption {
	default: number;
	from: number;
	to: number;
	step: number;
}

export type Options<T> = {
	[Property in keyof T]: RangeOption;
};

export type Entries<T> = { [K in keyof T]: [K, T[K]] }[keyof T][];
