import { onCleanup, onMount, useContext } from 'solid-js';
import { useScreenDimensions } from '../event/event_managers';
import { GameContext } from '../App';
import { BoshImage, BoshImages } from '../rider_data/rider_data';
import { EntityPoint, Line } from '../rust_interop/tauri_types';

function getAngleForPoints(p1: [number, number], p2: [number, number]) {
	return Math.atan2(p2[1] - p1[1], p2[0] - p1[0]);
}

function colorForIndex(index: string): string {
	if (index.startsWith('Bosh')) {
		return 'green';
	}
	if (index.startsWith('Sled')) {
		return 'blue';
	}

	return 'red';
}

function GameArea(props: { camera: { x: number; y: number }; zoom: number }) {
	let canvas: HTMLCanvasElement;

	let [width, height] = useScreenDimensions();
	let game = useContext(GameContext);

	const worldToScreen = (coord: [number, number]): [number, number] => {
		return [
			(coord[0] - props.camera.x) * props.zoom + width() / 2,
			(coord[1] - props.camera.y) * props.zoom + height() / 2,
		];
	};

	onMount(() => {
		const ctx = canvas.getContext('2d')!;

		function drawLine(line: Line) {
			const startCoord = worldToScreen(line.ends[0].location);
			const endCoord = worldToScreen(line.ends[1].location);
			const width = 2 * props.zoom;

			ctx.strokeStyle = 'black';
			ctx.fillStyle = 'black';
			ctx.lineWidth = width;
			ctx.beginPath();
			ctx.ellipse(...startCoord, width / 2, width / 2, 0, 0, Math.PI * 2);
			ctx.fill();
			ctx.beginPath();
			ctx.moveTo(...startCoord);
			ctx.lineTo(...endCoord);
			ctx.stroke();
			ctx.beginPath();
			ctx.ellipse(...endCoord, width / 2, width / 2, 0, 0, Math.PI * 2);
			ctx.fill();
		}

		function renderImageBetweenPoints(
			imageData: BoshImage,
			points: Record<string, EntityPoint>,
			pointIndices: [string, string],
		) {
			const p1 = points?.[pointIndices[0]]?.location;
			const p2 = points?.[pointIndices[1]]?.location;
			if (!p1 || !p2) {
				return;
			}
			const angleRads = getAngleForPoints(p1, p2);

			const img = new Image();
			img.src = imageData.data;
			ctx.save();
			ctx.translate(...worldToScreen(p1));
			ctx.rotate(angleRads);
			ctx.scale(props.zoom / 2, props.zoom / 2);
			ctx.translate(-imageData.anchor[0], -imageData.anchor[1]);
			ctx.drawImage(img, 0, 0);
			ctx.restore();
		}

		let frame = requestAnimationFrame((t) => loop(ctx, t));

		function loop(ctx: CanvasRenderingContext2D, t: number) {
			frame = requestAnimationFrame((newT) => loop(ctx, newT));

			ctx.clearRect(0, 0, canvas.width, canvas.height);
			for (const line of game.lines()) {
				drawLine(line);
			}
			for (const entity of game.entities()) {
				renderImageBetweenPoints(BoshImages.bosh, entity.points, [
					'BoshButt',
					'BoshShoulder',
				]);
				renderImageBetweenPoints(BoshImages.arm, entity.points, [
					'BoshShoulder',
					'BoshRightHand',
				]);
				renderImageBetweenPoints(BoshImages.arm, entity.points, [
					'BoshShoulder',
					'BoshLeftHand',
				]);
				renderImageBetweenPoints(BoshImages.leg, entity.points, [
					'BoshButt',
					'BoshRightFoot',
				]);
				renderImageBetweenPoints(BoshImages.leg, entity.points, [
					'BoshButt',
					'BoshLeftFoot',
				]);
				renderImageBetweenPoints(BoshImages.sled, entity.points, [
					'SledPeg',
					'SledRope',
				]);
			}
		}

		onCleanup(() => cancelAnimationFrame(frame));
	});

	// @ts-ignore (ref is special)
	return <canvas width={width()} height={height()} ref={canvas} />;
}

export default GameArea;
