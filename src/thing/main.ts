import init, { add_nums } from "wasm"

// console.log(wasm);
init().then(() => {
	console.log('init wasm-pack');
	alert(add_nums(1, 2));
});
