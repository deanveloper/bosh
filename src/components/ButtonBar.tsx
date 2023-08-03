import { JSX, useContext } from 'solid-js';
import HoldableButton from './HoldableButton';
import { GameContext } from '../rust_interop/game_manager';
import { openSingleFilePicker } from './filePicker';

function ButtonBar(props: { style?: JSX.CSSProperties }): JSX.Element {
	const gameManager = useContext(GameContext);

	return (
		<div style={props.style}>
			<HoldableButton
				disabled={gameManager.frame() <= 0}
				onClick={() => gameManager.setFrame(gameManager.frame() - 1)}
			>
				{'<'}
			</HoldableButton>
			<HoldableButton
				onClick={() => gameManager.setFrame(gameManager.frame() + 1)}
			>
				{'>'}
			</HoldableButton>
			<button
				onClick={() => {
					openSingleFilePicker()
						.then((path) => {
							path && gameManager.loadTrack(path);
						})
						.catch((err) => {
							console.error(err);
						})
				}}
			>
				Load Track
			</button>
		</div>
	);
}

export default ButtonBar;
