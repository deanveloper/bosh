import type { Component } from 'solid-js';
import { createSignal, ErrorBoundary, onCleanup, onMount } from 'solid-js';
import { keyDown } from './event/hotkeys';
import ButtonBar from './components/ButtonBar';
import GameArea from './components/GameArea';

import './App.module.css';
import { GameContext, GameManager } from './rust_interop/game_manager';
import { clear } from './rust_interop/tauri_commands';
import { useScroll } from './event/event_managers';

async function initializeGame(gameManager: GameManager) {
	await clear();

	await gameManager.addEntity({
		boshSled: { position: [0, 0] },
	});

	await gameManager.addLine({
		ends: [
			{
				location: [-50, 0],
			},
			{
				location: [50, 30],
			},
		],
		flipped: false,
		lineType: 'Normal',
	});
}

const App: Component = () => {
	const gameManager = new GameManager();
	const [zoom, setZoom] = createSignal(1);

	onMount(() => {
		initializeGame(gameManager).catch(console.error);
	});

	useScroll((ev) => {
		setZoom(zoom() + -ev.deltaY / 500);
		console.log(zoom());
	});

	// add event listener for keybinds
	window.addEventListener('keydown', keyDown);
	onCleanup(() => {
		window.removeEventListener('keydown', keyDown);
	});

	return (
		<div>
			<ErrorBoundary fallback={'error lol'}>
				<GameContext.Provider value={gameManager}>
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
							x:
								gameManager.entities()[0]?.points?.BoshButt
									?.location?.[0] ?? 0,
							y:
								gameManager.entities()[0]?.points?.BoshButt
									?.location?.[1] ?? 0,
						}}
						zoom={Math.exp(zoom())}
					/>
				</GameContext.Provider>
			</ErrorBoundary>
		</div>
	);
};

export default App;
