export function keyDown(ev: KeyboardEvent) {
	if (ev.key === 'r' && ev.ctrlKey) {
		window.location.reload();
	}
}
