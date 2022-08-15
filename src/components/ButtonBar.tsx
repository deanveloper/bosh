import { JSX, Setter } from 'solid-js';
import HoldableButton from './HoldableButton';

function ButtonBar(props: {
	style?: JSX.CSSProperties;
	frame: number;
	setFrame: Setter<number>;
}): JSX.Element {
	return (
		<div style={props.style}>
			<HoldableButton
				disabled={props.frame <= 0}
				onClick={() => props.setFrame(props.frame - 1)}
			>
				{'<'}
			</HoldableButton>
			<HoldableButton onClick={() => props.setFrame(props.frame + 1)}>
				{'>'}
			</HoldableButton>
			<span>(frame: {props.frame})</span>
		</div>
	);
}

export default ButtonBar;
