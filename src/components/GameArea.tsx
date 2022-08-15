import { onCleanup, onMount } from 'solid-js';
import { Entity, Line, PointIndex } from '../rust_interop/tauri_commands';
import { useScreenDimensions } from '../event/event_managers';

function colorForIndex(index: PointIndex): string {
	if (index.startsWith('Bosh')) {
		return 'green';
	}
	if (index.startsWith('Sled')) {
		return 'red';
	}

	return 'black';
}

function GameArea(props: {
	camera: { x: number; y: number };
	zoom: number;
	entities: Entity[];
	lines: Line[];
}) {
	let canvas: HTMLCanvasElement;

	let [width, height] = useScreenDimensions();

	const worldToScreen = (coord: [number, number]): [number, number] => {
		return [
			(coord[0] - props.camera.x) * props.zoom + width() / 2,
			(coord[1] - props.camera.y) * props.zoom + height() / 2,
		];
	};

	onMount(() => {
		const ctx = canvas.getContext('2d');
		if (ctx === null) {
			throw new Error('context cannot be null');
		}

		let frame = requestAnimationFrame((t) => loop(ctx, t));

		function loop(ctx: CanvasRenderingContext2D, t: number) {
			frame = requestAnimationFrame((newT) => loop(ctx, newT));

			ctx.clearRect(0, 0, canvas.width, canvas.height);
			for (const line of props.lines) {
				const startCoord = worldToScreen(line.ends[0]);
				const endCoord = worldToScreen(line.ends[1]);

				ctx.beginPath();
				ctx.strokeStyle = 'black';
				ctx.lineWidth = 4;
				ctx.moveTo(...startCoord);
				ctx.lineTo(...endCoord);
				ctx.stroke();
			}
			for (const entity of props.entities) {
				for (const [name, coord] of Object.entries(entity.points)) {
					const canvasCoord = worldToScreen(coord);

					ctx.fillStyle = colorForIndex(name as PointIndex);
					ctx.beginPath();
					ctx.ellipse(
						canvasCoord[0],
						canvasCoord[1],
						2,
						2,
						0,
						0,
						2 * Math.PI,
					);
					ctx.fill();
				}
			}
		}

		onCleanup(() => cancelAnimationFrame(frame));
	});

	return <canvas width={width()} height={height()} ref={canvas} />;
}

export default GameArea;
