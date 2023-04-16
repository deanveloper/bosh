import { JSX, mergeProps, ParentProps } from 'solid-js';
import { createDelayedRepeat } from '../event/event_managers';

const MOUSE1 = 0;

function HoldableButton(
	_preDefaultedProps: ParentProps<{
		disabled?: boolean;
		onClick?: (ev: MouseEvent) => void;
	}>,
): JSX.Element {
	const props = mergeProps({ disabled: false }, _preDefaultedProps);

	const [enableRepeat, disableRepeat] = createDelayedRepeat<MouseEvent>(
		200,
		1000 / 40,
		(ev) => {
			if (ev.button !== MOUSE1 || props.disabled) {
				return true;
			}

			props.onClick?.(ev);
		},
	);

	return (
		<button
			onMouseDown={enableRepeat}
			onMouseUp={disableRepeat}
			onMouseLeave={disableRepeat}
			disabled={props.disabled}
			onClick={props.onClick}
		>
			{props.children}
		</button>
	);
}

export default HoldableButton;
