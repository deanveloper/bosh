import type { Component } from 'solid-js';
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
} from './tauri_commands';
import ButtonBar from './components/ButtonBar';
import GameArea from './components/GameArea';

const App: Component = () => {
	const [frame, setFrame] = createSignal(0);
	const [riders, setRiders] = createSignal<Entity[]>([]);
	const [lines, setLines] = createSignal<Line[]>([]);

	onMount(() => {
		addEntity({
			entityType: 'BoshSled',
		}).catch(err => console.error(err));

		addLine({
			ends: [
				[-50, 0],
				[50, 30],
			],
			flipped: false,
			lineType: 'Normal',
		})
			.then(newLines => setLines(newLines))
			.catch(err => console.error(err));

		window.addEventListener('keydown', keyDown);

		onCleanup(() => {
			window.removeEventListener('keydown', keyDown);
		});
	});
	createEffect(() => {
		entityPositionsAt(frame())
			.then(pos => setRiders(pos))
			.catch(err => console.error(err));
	});

	return (
		<div>
			<ErrorBoundary fallback={'error lol'}>
				<ButtonBar
					style={{
						position: 'absolute',
						width: '100%',
						display: 'flex',
						justifyContent: 'center',
					}}
					frame={frame()}
					setFrame={setFrame}
				/>
				<GameArea
					camera={{ x: 0, y: 0 }}
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
