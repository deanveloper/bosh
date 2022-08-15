import type { Component, Setter } from 'solid-js';
import {
	createContext,
	createEffect,
	createMemo,
	createSignal,
	ErrorBoundary,
	JSX,
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
	const [entities, setEntities] = createSignal<Entity[]>([]);
	const [lines, setLines] = createSignal<Line[]>([]);

	onMount(() => {
		initializeGame(setLines);
	});

	// set riders when frame changes
	createEffect(() => {
		entityPositionsAt(frame())
			.then((pos) => setEntities(pos))
			.catch((err) => console.error(err));
	});

	// add event listener for keybinds
	window.addEventListener('keydown', keyDown);
	onCleanup(() => {
		window.removeEventListener('keydown', keyDown);
	});

	const game = createMemo(() => ({
		frame: frame,
		setFrame: setFrame,
		lines: lines,
		entities: entities,
	}));

	return (
		<div>
			<ErrorBoundary fallback={'error lol'}>
				<GameContext.Provider value={game()}>
					<ButtonBar
						style={{
							position: 'absolute',
							width: '100%',
							display: 'flex',
							'justify-content': 'center',
						}}
					/>
					<GameArea
						camera={{
							x: entities()[0]?.points?.BoshButt?.[0] ?? 0,
							y: entities()[0]?.points?.BoshButt?.[1] ?? 0,
						}}
						zoom={3}
					/>
				</GameContext.Provider>
			</ErrorBoundary>
		</div>
	);
};

export const GameContext = createContext<{
	frame: JSX.Accessor<number>;
	setFrame: Setter<number>;
	lines: JSX.Accessor<Line[]>;
	entities: JSX.Accessor<Entity[]>;
}>({
	frame: () => 0,
	setFrame: (() => undefined) as Setter<number>,
	lines: () => [],
	entities: () => [],
});

export default App;
