export interface Option<N extends string> {
	name: N;
	value: number;
	description: string;
}

export type OptionObj<N extends string> = Record<N, number>;
