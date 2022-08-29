import {createSignal, JSX, onCleanup, onMount} from 'solid-js';

/**
 * From calling the returned `enable` function, it will have a short delay,
 * and then call the handler. Then it will repeatedly call the handler until `disable` is called.
 *
 * @return
 */
export function createDelayedRepeat<T>(
	delay: number,
	repeat: number,
	handler: (ev: T) => true | void,
): [enable: (ev: T) => void, disable: () => void] {
	const [intervalId, setIntervalId] = createSignal(-1);

	function enable(t: T) {
		const newInterval = setTimeout(() => {
			const newInterval = setInterval(() => {
				if (handler?.(t) === true) {
					clearInterval(intervalId());
					return;
				}
			}, repeat);

			clearInterval(intervalId());
			setIntervalId(newInterval);
		}, delay);

		clearInterval(intervalId());
		setIntervalId(newInterval);
	}

	function disable() {
		clearInterval(intervalId());
	}

	return [enable, disable];
}

export function useScreenDimensions(): [
	w: JSX.Accessor<number>,
	h: JSX.Accessor<number>,
] {
	const [width, setWidth] = createSignal(window.innerWidth);
	const [height, setHeight] = createSignal(window.innerHeight);

	onMount(() => {
		function handleResize(ev: UIEvent) {
			setWidth(window.innerWidth);
			setHeight(window.innerHeight);
		}

		window.addEventListener('resize', handleResize);

		onCleanup(() => {
			window.removeEventListener('resize', handleResize);
		});
	});

	return [width, height];
}

export function useScroll(onScroll: (ev: WheelEvent) => void) {
	onMount(() => {
		window.addEventListener('wheel', onScroll);

		onCleanup(() => window.removeEventListener('wheel', onScroll));
	})
}