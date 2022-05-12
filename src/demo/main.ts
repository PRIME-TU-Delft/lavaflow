import init, * as wasm from "wasm"
import * as THREE from "three"
import {GLTFLoader} from 'three/examples/jsm/loaders/GLTFLoader.js'
import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls.js'



let hc_level_curves = [[[798, 101], [789, 92], [777, 84], [753, 71], [743, 67], [730, 65], [717, 63], [703, 63], [691, 63], [678, 66], [665, 70], [651, 77], [638, 85], [627, 93], [618, 102], [613, 111], [607, 120], [599, 136], [594, 148], [589, 157], [583, 170], [577, 184], [573, 194], [568, 209], [563, 221], [560, 234], [554, 246], [550, 259], [544, 272], [536, 285], [528, 297], [520, 310], [511, 323], [504, 331], [493, 343], [485, 350], [470, 362], [456, 374], [441, 384], [425, 394], [409, 404], [398, 409], [383, 417], [368, 424], [350, 433], [333, 439], [314, 446], [295, 453], [278, 461], [262, 467], [247, 474], [236, 479], [221, 488], [210, 494], [195, 504], [186, 509], [174, 519], [158, 533], [150, 541], [141, 552], [134, 562], [128, 574], [122, 585], [117, 596], [113, 606], [111, 617], [111, 630], [114, 640], [117, 651], [122, 661], [129, 671], [136, 680], [145, 689], [154, 696], [164, 703], [173, 708], [187, 715], [204, 720], [225, 724], [250, 727], [271, 729], [294, 732], [309, 732], [333, 734], [359, 734], [383, 735], [406, 736], [428, 736], [449, 737], [467, 737], [484, 739], [502, 740], [520, 741], [542, 742], [563, 744], [592, 747], [607, 748], [620, 749], [636, 751], [652, 753], [680, 755], [691, 755], [709, 757], [738, 759], [765, 759], [790, 759], [815, 757], [835, 754], [846, 753], [866, 748], [878, 744], [888, 738], [898, 731], [911, 718], [918, 710], [926, 700], [936, 684], [945, 667], [949, 654], [953, 643], [955, 633], [957, 620], [959, 609], [960, 594], [961, 582], [962, 571], [962, 558], [959, 545], [957, 535], [953, 523], [948, 509], [943, 495], [938, 479], [934, 468], [931, 457], [926, 445], [921, 431], [917, 414], [912, 400], [905, 386], [901, 375], [896, 360], [893, 348], [890, 338], [886, 325], [881, 315], [879, 305], [874, 292], [871, 281], [867, 267], [864, 253], [861, 239], [858, 228], [855, 215], [850, 203], [846, 191], [841, 181], [836, 170], [830, 161], [824, 151], [819, 142]], [[768, 324], [760, 316], [741, 313], [723, 312], [712, 314], [697, 317], [684, 322], [675, 327], [664, 334], [656, 342], [649, 350], [642, 358], [634, 367], [625, 376], [618, 384], [611, 393], [602, 401], [593, 409], [577, 423], [565, 432], [552, 440], [538, 448], [523, 456], [511, 461], [497, 467], [485, 471], [466, 478], [455, 481], [437, 487], [418, 493], [400, 499], [383, 505], [368, 511], [357, 516], [346, 521], [330, 528], [316, 535], [307, 540], [295, 545], [282, 554], [270, 563], [256, 574], [248, 581], [236, 595], [231, 604], [229, 616], [232, 629], [239, 639], [250, 647], [264, 653], [274, 656], [285, 658], [297, 660], [309, 662], [324, 663], [338, 665], [353, 666], [366, 668], [376, 669], [390, 670], [405, 672], [421, 673], [434, 674], [449, 675], [467, 677], [481, 677], [494, 679], [509, 681], [519, 682], [533, 683], [546, 684], [558, 684], [570, 684], [586, 684], [601, 684], [617, 685], [632, 686], [652, 688], [666, 691], [681, 692], [694, 694], [704, 696], [718, 698], [732, 700], [746, 702], [758, 704], [769, 706], [781, 707], [793, 708], [805, 709], [815, 710], [829, 710], [843, 707], [853, 704], [863, 700], [876, 694], [886, 688], [899, 680], [910, 669], [917, 659], [924, 649], [928, 637], [931, 625], [932, 614], [934, 603], [936, 589], [936, 577], [934, 563], [931, 552], [927, 542], [921, 529], [916, 518], [909, 507], [901, 495], [894, 485], [887, 476], [878, 467], [867, 455], [857, 445], [847, 433], [836, 421], [824, 411], [817, 403], [809, 390], [804, 381], [798, 370], [793, 361], [789, 350], [785, 340]], [[744, 459], [734, 450], [722, 447], [706, 447], [691, 450], [678, 454], [667, 459], [658, 466], [649, 475], [641, 484], [634, 492], [624, 504], [614, 511], [601, 518], [587, 526], [574, 533], [558, 538], [546, 542], [527, 548], [511, 555], [501, 560], [489, 567], [479, 575], [468, 587], [463, 599], [467, 611], [480, 618], [492, 622], [508, 624], [520, 625], [533, 627], [548, 627], [559, 629], [573, 630], [584, 630], [597, 632], [608, 632], [618, 633], [628, 635], [639, 635], [649, 636], [660, 638], [670, 641], [682, 642], [696, 646], [708, 648], [722, 651], [732, 652], [743, 655], [755, 657], [769, 659], [784, 661], [797, 662], [811, 663], [822, 663], [833, 661], [847, 658], [857, 654], [868, 646], [877, 637], [883, 626], [885, 613], [885, 599], [882, 588], [878, 575], [872, 564], [864, 552], [858, 543], [851, 535], [843, 528], [833, 519], [822, 510], [813, 502], [803, 494], [793, 486], [782, 477], [774, 469]], [[833, 561], [826, 550], [817, 542], [808, 537], [794, 529], [785, 524], [770, 518], [759, 514], [743, 513], [728, 514], [717, 518], [705, 523], [695, 529], [686, 537], [681, 546], [678, 556], [677, 566], [679, 578], [682, 588], [689, 598], [696, 606], [705, 613], [717, 620], [732, 625], [744, 627], [756, 628], [767, 629], [779, 631], [791, 632], [805, 633], [817, 632], [828, 629], [837, 624], [845, 616], [850, 606], [852, 596], [850, 586], [845, 575], [840, 566], [833, 557]], [[789, 569], [780, 563], [770, 561], [758, 561], [746, 562], [737, 568], [736, 579], [744, 588], [754, 593], [765, 597], [776, 599], [786, 598], [793, 590], [798, 580]]]
let hc_parent_relations = [-1, 0, 1, 2, 3]

init().then(() => {
	console.log('init wasm-pack');

	let tree = new wasm.OpenCVTree({ "pixels_per_curve": hc_level_curves, "parent_relations": hc_parent_relations });

	let settings = new wasm.ModelGenerationSettings(5.0, 50, 50, 50.0, 1.0);

	console.log(tree.debug());
	console.log(settings.debug());

	let repetitions = 2;
	let strength_positive = 0.7;
	let strength_negative = 0.7;
	let coverage = 8;
	let svc_weight = 50;

	let rows = 60;
	let columns = 60;
	let contour_margin = 20.0;

	let gltf = wasm.generate_3d_model(tree, settings, repetitions, strength_positive, strength_negative, coverage, svc_weight, rows, columns, contour_margin);
	console.log(gltf);

	// View the GLTF in the browser
	const scene = new THREE.Scene();
	const camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );
	const renderer = new THREE.WebGLRenderer();
	const controls = new OrbitControls( camera, renderer.domElement );

	renderer.setFaceCulling(false);
	renderer.setClearColor( 0xbbbfc9, 1 );
	renderer.setSize( window.innerWidth, window.innerHeight );
	document.body.appendChild( renderer.domElement );

	const geometry = new THREE.BoxGeometry();
	const material = new THREE.MeshBasicMaterial( { color: 0xff0000 } );
	const cube = new THREE.Mesh( geometry, material );
	//.scene.add( cube );

	camera.position.y = 150;
	camera.position.x = 100;
	camera.position.z = 100;
	controls.update();



	const light = new THREE.PointLight( 0xffffff, 1, 0 );
	light.position.set( 100, 150, 100 );
	scene.add( light );

	const light2 = new THREE.AmbientLight( 0x999999, 1 ); // soft white light
	scene.add( light2 );


	let loader = new GLTFLoader();
	loader.load(
		"data:text/plain;base64," + btoa(gltf),  // Data URI with the GLTF file's content
		// called when the resource is loaded
		function ( gltf ) {
			gltf.scene.scale.set(0.1, 0.1, 0.1);  // Scale the model
			gltf.scene.children[0].material.side = THREE.DoubleSide;  // "Invert" the mode sides

			gltf.scene.traverse( child => {

				if ( child.isMesh ) child.material.flatShading = THREE.SmoothShading;
				if ( child.material ) child.material.metalness = 0.5;
			
			} );

			gltf.scene.traverse( function ( node ) {

				if ( node.isMesh || node.isLight ) node.castShadow = true;
				if ( node.isMesh || node.isLight ) node.receiveShadow = true;

			} );

			gltf.scene.position.x = -60;
			gltf.scene.position.z = -50;

			scene.add( gltf.scene );

			gltf.animations; // Array<THREE.AnimationClip>
			gltf.scene; // THREE.Group
			gltf.scenes; // Array<THREE.Group>
			gltf.cameras; // Array<THREE.Camera>
			gltf.asset; // Object

		},
		// called while loading is progressing
		function ( xhr ) {

			console.log( ( xhr.loaded / xhr.total * 100 ) + '% loaded' );
			console.log("xhr.total = " + xhr.total);


		},
		// called when loading has errors
		function ( error ) {

			console.log( 'An error happened' );

		}

	);

	renderer.shadowMap.enabled = true


	function animate() {
		requestAnimationFrame( animate );
		controls.update();
		renderer.render( scene, camera );
	};
	animate();

});
