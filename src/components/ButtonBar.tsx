import { JSX, useContext } from 'solid-js';
import HoldableButton from './HoldableButton';
import { GameContext } from '../App';

function ButtonBar(props: { style?: JSX.CSSProperties }): JSX.Element {
	const game = useContext(GameContext);

	return (
		<div style={props.style}>
			<HoldableButton
				disabled={game.frame() <= 0}
				onClick={() => game.setFrame(game.frame() - 1)}
			>
				{'<'}
			</HoldableButton>
			<HoldableButton onClick={() => game.setFrame(game.frame() + 1)}>
				{'>'}
			</HoldableButton>
			<span>(frame: {game.frame})</span>
		</div>
	);
}

export default ButtonBar;
