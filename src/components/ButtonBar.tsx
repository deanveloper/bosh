import { JSX, useContext } from 'solid-js';
import HoldableButton from './HoldableButton';
import { GameContext } from '../rust_interop/game_manager';

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
				onClick={() =>
					gameManager.loadTrack(
						'C:/Users/dean/Desktop/uh oh.trk',
					)
				}
			>
				Load Track
			</button>
		</div>
	);
}

export default ButtonBar;
