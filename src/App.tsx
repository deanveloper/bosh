import type { Component, Setter } from 'solid-js';
import {
	createEffect,
	createSignal,
	ErrorBoundary,
	onCleanup,
	onMount,
} from 'solid-js';
import { keyDown } from './hotkeys';

import {
	addEntity,
	addLine,
	Entity,
	entityPositionsAt,
	Line,
} from './rust_interop/tauri_commands';
import ButtonBar from './components/ButtonBar';
import GameArea from './components/GameArea';

function initializeGame(setLines: Setter<Line[]>) {
	addEntity({
		entityType: 'BoshSled',
	}).catch((err) => console.error(err));

	addLine({
		ends: [
			[-50, 0],
			[50, 30],
		],
		flipped: false,
		lineType: 'Normal',
	})
		.then((newLines) => setLines(newLines))
		.catch((err) => console.error(err));
}

const App: Component = () => {
	const [frame, setFrame] = createSignal(0);
	const [riders, setRiders] = createSignal<Entity[]>([]);
	const [lines, setLines] = createSignal<Line[]>([]);

	onMount(() => {
		initializeGame(setLines);
	});

	// set riders when frame changes
	createEffect(() => {
		entityPositionsAt(frame())
			.then((pos) => setRiders(pos))
			.catch((err) => console.error(err));
	});

	// add event listener for keybinds
	window.addEventListener('keydown', keyDown);
	onCleanup(() => {
		window.removeEventListener('keydown', keyDown);
	});

	return (
		<div>
			<ErrorBoundary fallback={'error lol'}>
				<ButtonBar
					style={{
						position: 'absolute',
						width: '100%',
						display: 'flex',
						'justify-content': 'center',
					}}
					frame={frame()}
					setFrame={setFrame}
				/>
				<GameArea
					camera={{
						x: riders()[0]?.points?.BoshButt?.[0] ?? 0,
						y: riders()[0]?.points?.BoshButt?.[1] ?? 0,
					}}
					width={500}
					height={500}
					zoom={3}
					entities={riders()}
					lines={lines()}
				/>
			</ErrorBoundary>
		</div>
	);
};

export default App;
