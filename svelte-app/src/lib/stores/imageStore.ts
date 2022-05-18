import { writable } from 'svelte/store';

function createImage() {
	const { subscribe, set, update } = writable<string>(undefined);

	return {
		subscribe,
		setImage: (videoSource: HTMLVideoElement) => {
			const canvas = document.createElement('canvas');

			const ctx = canvas.getContext('2d');

			if (!ctx) return;
			ctx.drawImage(videoSource, 0, 0, canvas.width, canvas.height);
			const image = canvas.toDataURL('image/jpg');
			set(image);
		}
	};
}

export const imageUrl = createImage();
