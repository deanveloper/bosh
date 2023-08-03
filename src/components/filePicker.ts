import * as dialog from "@tauri-apps/api/dialog";

type PickFileResult = string | undefined;

export async function openSingleFilePicker(): Promise<PickFileResult> {
	const fileName = await dialog.open({
		title: 'track picker',
		multiple: false,
		directory: false,
	});
	if (Array.isArray(fileName)) {
		throw new Error('single file picker picked multiple files');
	}
	if (!fileName) {
		return;
	}
	return fileName;
}
