import { Component, JSX } from 'solid-js';
import ButtonWithHold from './ButtonWithHold';

const ButtonBar: Component<{
	style?: JSX.CSSProperties;
	frame: number;
	setFrame: (i: number) => void;
}> = props => {
	return (
		<div style={props.style}>
			<ButtonWithHold
				disabled={props.frame <= 0}
				onClick={() => props.setFrame(props.frame - 1)}
			>
				{'<'}
			</ButtonWithHold>
			<ButtonWithHold onClick={() => props.setFrame(props.frame + 1)}>
				{'>'}
			</ButtonWithHold>
			<span>(frame: {props.frame})</span>
		</div>
	);
};

export default ButtonBar;
