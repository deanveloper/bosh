const arm = await fetch(`/src/assets/arm.svg`)
	.then((res) => res.blob())
	.then((blob) => URL.createObjectURL(blob));
const bosh = await fetch(`/src/assets/bosh.svg`)
	.then((res) => res.blob())
	.then((blob) => URL.createObjectURL(blob));
const leg = await fetch(`/src/assets/leg.svg`)
	.then((res) => res.blob())
	.then((blob) => URL.createObjectURL(blob));
const sled = await fetch(`/src/assets/sled.svg`)
	.then((res) => res.blob())
	.then((blob) => URL.createObjectURL(blob));

export type BoshImage = { anchor: [number, number]; data: string };
export const BoshImages: Record<string, BoshImage> = {
	arm: {
		anchor: [1, 1.5],
		data: arm,
	},
	bosh: {
		anchor: [0, 7],
		data: bosh,
	},
	leg: {
		anchor: [1, 3.5],
		data: leg,
	},
	sled: {
		anchor: [1, 4.5],
		data: sled,
	},
};
