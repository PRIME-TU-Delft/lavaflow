export default class LavaError extends Error {
	constructor(public title: string, message: string) {
		super(message);
		this.title = title;
	}

	toString() {
		return `${this.name}: ${this.message}`;
	}
}
