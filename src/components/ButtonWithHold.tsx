import {
	createEffect,
	createSignal,
	mergeProps,
	ParentComponent,
} from 'solid-js';

const ButtonWithHold: ParentComponent<{
	disabled?: boolean;
	onClick?: (ev: MouseEvent) => void;
}> = props => {
	props = mergeProps({ disabled: false }, props);

	const [intervalId, setIntervalId] = createSignal(-1);

	function onMouseDown(ev: MouseEvent) {
		if (ev.button !== 0) {
			return;
		}
		const newInterval = setTimeout(() => {
			const newInterval = setInterval(() => {
				props.onClick?.(ev);
			}, 50);
			clearInterval(intervalId());
			setIntervalId(newInterval);
		}, 250);
		clearInterval(intervalId());
		setIntervalId(newInterval);
	}

	function onMouseUp() {
		clearInterval(intervalId());
	}

	createEffect(() => {
		if (props.disabled) {
			clearInterval(intervalId());
		}
	});

	return (
		<button
			onMouseDown={onMouseDown}
			onMouseUp={onMouseUp}
			disabled={props.disabled}
			onClick={props.onClick}
		>
			{props.children}
		</button>
	);
};

export default ButtonWithHold;
