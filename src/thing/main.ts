import init, * as wasm from "wasm"

init().then(() => {
	console.log('init wasm-pack');

	let p1 = new wasm.Point(1, 2, 3);
	let p2 = new wasm.Point(3, 2, 1);
	console.log(`distance = ${p1.distance(p2)}`);
});
