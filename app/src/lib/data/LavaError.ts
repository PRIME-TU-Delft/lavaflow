export default class LavaError extends Error {
	constructor(public title: string, message: string) {
		super(message);
		this.title = title;
		console.error("[LavaError] " + title + ": " + message)
	}

	toString() {
		return `${this.name}: ${this.message}`;
	}
}
